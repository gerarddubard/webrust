// webrust/static/script.js
let currentInputId = null;
let currentInputElement = null;
let lastOutputLength = 0;
let processingInput = false;
let currentErrorElement = null;
let currentInputContainer = null;

function showError(message) {
    if (currentErrorElement) currentErrorElement.remove();
    currentErrorElement = document.createElement('div');
    currentErrorElement.className = 'error-message';
    currentErrorElement.textContent = `❌ ${message}`;
    if (currentInputContainer) currentInputContainer.appendChild(currentErrorElement);
}

function clearError() {
    if (currentErrorElement) {
        currentErrorElement.remove();
        currentErrorElement = null;
    }
}

function processLatexLine(line) {
    if (line.startsWith('LATEX_DISPLAY:')) {
        const formula = line.substring(14);
        return `<div class="latex-display">$$${formula}$$</div>`;
    } else if (line.startsWith('LATEX_INLINE:')) {
        const formula = line.substring(13);
        return `<span class="latex-inline">$${formula}$</span>`;
    }
    return line;
}

function renderMathJax(container) {
    // Utilisation de la notation bracket pour éviter les warnings
    if (typeof window !== 'undefined' && window.MathJax) {
        if (window.MathJax['typesetPromise']) {
            // MathJax v3
            window.MathJax['typesetPromise']([container]).catch(function (err) {
                console.log('MathJax v3 error:', err.message);
            });
        } else if (window.MathJax['Hub'] && window.MathJax['Hub']['Queue']) {
            // MathJax v2
            window.MathJax['Hub']['Queue'](["Typeset", window.MathJax['Hub'], container]);
        }
    }
}

function updateDisplay() {
    if (processingInput) return;
    fetch('/api/state')
        .then(response => response.json())
        .then(data => {
            const terminal = document.getElementById('terminal');
            if (data.output.length !== lastOutputLength) {
                lastOutputLength = data.output.length;
                let currentInputValue = '';
                if (currentInputElement && !currentInputElement.disabled) {
                    currentInputValue = currentInputElement.value;
                }
                terminal.innerHTML = '';
                currentInputElement = null;
                currentInputId = null;
                currentErrorElement = null;
                currentInputContainer = null;
                let needsMathJaxRender = false;
                for (let i = 0; i < data.output.length; i++) {
                    const line = data.output[i];
                    if (line.startsWith('INPUT_REQUEST:')) {
                        const parts = line.split(':');
                        const inputId = parts[1];
                        const prompt = parts.slice(2).join(':');
                        const nextLine = i + 1 < data.output.length ? data.output[i + 1] : null;
                        const isCompleted = nextLine && !nextLine.startsWith('INPUT_REQUEST:') && !nextLine.startsWith('PROGRAM_FINISHED');
                        if (isCompleted) {
                            const completedDiv = document.createElement('div');
                            completedDiv.className = 'terminal-line';
                            completedDiv.innerHTML = `<span class="input-prompt">${prompt}</span> <span class="completed-input">${processLatexLine(nextLine)}</span>`;
                            terminal.appendChild(completedDiv);
                            i++;
                        } else {
                            currentInputId = inputId;
                            currentInputContainer = document.createElement('div');
                            currentInputContainer.className = 'input-container';
                            const inputDiv = document.createElement('div');
                            inputDiv.className = 'input-line';
                            const promptSpan = document.createElement('span');
                            promptSpan.className = 'input-prompt';
                            promptSpan.innerHTML = prompt + ' ';
                            const inputField = document.createElement('input');
                            inputField.className = 'user-input';
                            inputField.type = 'text';
                            inputField.value = currentInputValue;
                            currentInputElement = inputField;
                            inputField.addEventListener('keypress', function(e) {
                                if (e.key === 'Enter') {
                                    const value = inputField.value.trim();
                                    if (value) {
                                        fetch('/api/validate', {
                                            method: 'POST',
                                            headers: { 'Content-Type': 'application/json' },
                                            body: JSON.stringify({ id: currentInputId, value: value })
                                        })
                                            .then(response => response.json())
                                            .then(result => {
                                                if (result.valid) {
                                                    clearError();
                                                    submitInput();
                                                } else {
                                                    showError(result.error);
                                                    inputField.value = '';
                                                    inputField.focus();
                                                }
                                            });
                                    } else {
                                        showError("Please enter a value");
                                        inputField.focus();
                                    }
                                }
                            });
                            inputField.addEventListener('input', function() {
                                if (inputField.value.trim() && currentErrorElement) clearError();
                            });
                            inputDiv.appendChild(promptSpan);
                            inputDiv.appendChild(inputField);
                            currentInputContainer.appendChild(inputDiv);
                            terminal.appendChild(currentInputContainer);
                            setTimeout(() => {
                                inputField.focus();
                                inputField.setSelectionRange(inputField.value.length, inputField.value.length);
                            }, 50);
                        }
                    } else if (!line.startsWith('INPUT_REQUEST:')) {
                        const prevLine = i > 0 ? data.output[i - 1] : null;
                        if (!prevLine || !prevLine.startsWith('INPUT_REQUEST:')) {
                            const lineDiv = document.createElement('div');
                            lineDiv.className = 'terminal-line';
                            lineDiv.innerHTML = processLatexLine(line);
                            terminal.appendChild(lineDiv);
                            if (line.startsWith('LATEX_DISPLAY:') || line.startsWith('LATEX_INLINE:')) {
                                needsMathJaxRender = true;
                            }
                        }
                    }
                }
                if (needsMathJaxRender) { renderMathJax(terminal); }
                terminal.scrollTop = terminal.scrollHeight;
            }
        });
}

function submitInput() {
    if (currentInputId && currentInputElement && !processingInput) {
        const value = currentInputElement.value;
        if (value.trim()) {
            processingInput = true;
            currentInputElement.disabled = true;
            fetch('/api/input', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ id: currentInputId, value: value })
            })
                .then(() => {
                    processingInput = false;
                    currentInputId = null;
                    currentInputElement = null;
                    currentErrorElement = null;
                    currentInputContainer = null;
                    lastOutputLength = 0;
                })
                .catch(() => { processingInput = false; });
        }
    }
}

setInterval(updateDisplay, 300);
updateDisplay();