/* webrust/static/main.js */
/** @ts-check */
(() => {
    let currentInputId = null, currentInputElement = null, currentErrorElement = null, currentInputContainer = null;
    let renderedCount = 0, processingInput = false, inFlight = null;
    const term = document.getElementById("terminal");
    const hasMath = (s) => /\$[^$]*\$|\\\(|\\\[|\\begin\{/.test(s || "");
    const showError = (m) => { currentErrorElement?.remove(); currentErrorElement = document.createElement("div"); currentErrorElement.className = "error-message"; currentErrorElement.textContent = "❌ " + m; currentInputContainer?.appendChild(currentErrorElement); };
    const clearError = () => { if (currentErrorElement) { currentErrorElement.remove(); currentErrorElement = null; } };
    const renderMathBatch = (nodes) => {
        if (!nodes.length) return;
        const mj = window.MathJax;
        if (!mj) { setTimeout(() => renderMathBatch(nodes), 100); return; }
        const tp = mj["typesetPromise"], ts = mj["typeset"];
        if (typeof tp === "function") { tp.call(mj, nodes).catch(() => {}); return; }
        if (typeof ts === "function") { try { ts.call(mj, nodes); } catch {} return; }
        setTimeout(() => renderMathBatch(nodes), 100);
    };
    const runInlineScriptsBatch = (nodes) => {
        if (!nodes.length) return;
        requestAnimationFrame(() => {
            for (const n of nodes) {
                const scripts = n.querySelectorAll("script");
                for (const s of scripts) { try { (new Function(String(s.textContent)))(); } catch {} }
            }
        });
    };
    const appendObjectStage = (p, frag) => {
        let d = document.getElementById("ostage-" + p.id);
        if (!d) { d = document.createElement("div"); d.id = "ostage-" + p.id; d.className = "object-stage"; frag.appendChild(d); }
        window.webrustTurtle.handleStage(p);
    };
    const submitInput = () => {
        if (!currentInputId || !currentInputElement || processingInput) return;
        const v = currentInputElement.value;
        const id = currentInputId;
        if (!v.trim()) return;
        processingInput = true;
        currentInputElement.disabled = true;

        const promptSpan = currentInputContainer.querySelector(".input-prompt");
        const promptText = promptSpan ? promptSpan.textContent : "Input:";

        fetch("/api/input", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ id, value: v }) })
            .then(() => {
                if (currentInputContainer && currentInputContainer.parentNode) {
                    const completedDiv = document.createElement("div");
                    completedDiv.className = "terminal-line";
                    completedDiv.innerHTML = `<span class="input-prompt">${promptText}</span> <span class="completed-input">${v}</span>`;
                    currentInputContainer.parentNode.insertBefore(completedDiv, currentInputContainer);
                    currentInputContainer.remove();
                    term.scrollTop = term.scrollHeight;
                }
                processingInput = false;
                currentInputId = null;
                currentInputElement = null;
                currentErrorElement = null;
                currentInputContainer = null;
            })
            .catch(() => {
                processingInput = false;
                if (currentInputElement) currentInputElement.disabled = false;
            });
    };
    const buildInputRow = (id, prompt, restore, frag, focusTasks) => {
        currentInputId = id;
        currentInputContainer = document.createElement("div");
        currentInputContainer.className = "input-container";
        const row = document.createElement("div");
        row.className = "input-line";
        const span = document.createElement("span");
        span.className = "input-prompt";
        span.innerHTML = prompt + " ";
        const inp = document.createElement("input");
        inp.className = "user-input";
        inp.type = "text";
        inp.value = restore;
        currentInputElement = inp;
        inp.addEventListener("keypress", (e) => {
            if (e.key !== "Enter") return;
            const val = inp.value.trim();
            if (!val) { showError("Please enter a value"); inp.focus(); return; }
            fetch("/api/validate", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ id, value: val }) })
                .then((r) => r.json())
                .then((res) => { if (res?.valid) { clearError(); submitInput(); } else { showError(res?.error || "Invalid value"); inp.value = ""; inp.focus(); } })
                .catch(() => { showError("Validation failed"); inp.focus(); });
        });
        inp.addEventListener("input", () => { if (inp.value.trim() && currentErrorElement) clearError(); });
        row.appendChild(span); row.appendChild(inp); currentInputContainer.appendChild(row); frag.appendChild(currentInputContainer);
        focusTasks.push(() => { inp.focus(); const L = inp.value.length; try { inp.setSelectionRange(L, L); } catch {} });
    };
    const updateDisplay = () => {
        if (processingInput || document.hidden) return;
        inFlight?.abort?.();
        const ctrl = new AbortController();
        inFlight = ctrl;
        fetch("/api/state", { signal: ctrl.signal })
            .then((r) => r.json())
            .then((data) => {
                if (!term || !Array.isArray(data?.output)) return;

                if (renderedCount > data.output.length) {
                    term.innerHTML = "";
                    renderedCount = 0;
                }
                if (renderedCount === data.output.length) return;

                const frag = document.createDocumentFragment();
                const mathNodes = [];
                const scriptNodes = [];
                const focusTasks = [];
                let i = renderedCount;
                let restore = "";

                if (currentInputElement && !currentInputElement.disabled) {
                    restore = currentInputElement.value;
                }
                if (currentInputContainer) {
                    currentInputContainer.remove();
                }
                currentInputElement = null; currentInputId = null; currentErrorElement = null; currentInputContainer = null;

                for (; i < data.output.length; i++) {
                    const line = String(data.output[i] ?? "");

                    if (line.startsWith("OBJECT_STAGE:")) { appendObjectStage(JSON.parse(line.slice(13)), frag); continue; }
                    if (line.startsWith("OBJECT_CMDS:")) { const p = JSON.parse(line.slice(12)); window.webrustTurtle.handleCmds(p); continue; }
                    if (line.startsWith("OBJECT_GROUP:")) { const p = JSON.parse(line.slice(13)); window.webrustTurtle.handleGroup(p); continue; }

                    if (line.startsWith("SIMPLE_TABLE:")) {
                        const html = line.substring(13);
                        const div = document.createElement("div");
                        div.className = "table-container";
                        div.innerHTML = html;
                        frag.appendChild(div);
                        if (hasMath(html)) mathNodes.push(div);
                        continue;
                    }

                    if (line.startsWith("INPUT_REQUEST:")) {
                        const parts = line.split(":");
                        const id = parts[1];
                        const prompt = parts.slice(2).join(":");
                        buildInputRow(id, prompt, restore, frag, focusTasks);
                        continue;
                    }

                    const div = document.createElement("div");
                    div.className = "terminal-line";
                    div.innerHTML = line;
                    frag.appendChild(div);
                    if (hasMath(line)) mathNodes.push(div);
                    if (line.includes("<script>")) scriptNodes.push(div);
                }

                term.appendChild(frag);
                for (const f of focusTasks) f();
                if (mathNodes.length) renderMathBatch(mathNodes);
                if (scriptNodes.length) runInlineScriptsBatch(scriptNodes);

                renderedCount = data.output.length;
                term.scrollTop = term.scrollHeight;
            })
            .catch((err) => { if (err?.name !== "AbortError") console.error("Error fetching state:", err); })
            .finally(() => { inFlight = null; });
    };
    let rafScheduled = false;
    const scheduleUpdate = () => {
        if (!rafScheduled && !processingInput && !document.hidden) {
            rafScheduled = true;
            requestAnimationFrame(() => { rafScheduled = false; updateDisplay(); });
        }
    };
    setInterval(scheduleUpdate, 100);
    scheduleUpdate();
})();