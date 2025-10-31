/* webrust/static/main.js */
(()=>{
    const term=document.getElementById("terminal"),root=document.documentElement;
    let curId=null,curInp=null,curErr=null,curCtr=null,rendCnt=0,procInp=false,inFlight=null;

    fetch("/api/style").then(r=>r.json()).then(s=>{
        root.style.setProperty('--bg',s.bg);
        root.style.setProperty('--color',s.color);
        root.style.setProperty('--font',s.font);
        root.style.setProperty('--size',s.size);
    }).catch(e=>console.error("Style error:",e));

    const hasMath=s=>/\$[^$]*\$|\\\(|\\\[|\\begin\{/.test(s||"");
    const showErr=m=>{curErr?.remove();curErr=document.createElement("div");curErr.className="error-message";curErr.textContent="❌ "+m;curCtr?.appendChild(curErr);};
    const clrErr=()=>{if(curErr){curErr.remove();curErr=null;}};
    const renderMath=nodes=>{
        if(!nodes.length)return;
        const mj=window.MathJax;
        if(!mj){setTimeout(()=>renderMath(nodes),100);return;}
        const tp=mj["typesetPromise"],ts=mj["typeset"];
        if(typeof tp==="function"){tp.call(mj,nodes).catch(()=>{});return;}
        if(typeof ts==="function"){try{ts.call(mj,nodes);}catch{}return;}
        setTimeout(()=>renderMath(nodes),100);
    };
    const runScripts=nodes=>{
        if(!nodes.length)return;
        requestAnimationFrame(()=>{
            for(const n of nodes){
                const scripts=n.querySelectorAll("script");
                for(const s of scripts){try{(new Function(String(s.textContent)))();}catch{}}
            }
        });
    };
    const appendStage=(p,frag)=>{
        let d=document.getElementById("ostage-"+p.id);
        if(!d){d=document.createElement("div");d.id="ostage-"+p.id;d.className="object-stage";frag.appendChild(d);}
        window.webrustTurtle.handleStage(p);
    };
    const submit=()=>{
        if(!curId||!curInp||procInp)return;
        const v=curInp.value,id=curId;
        if(!v.trim())return;
        procInp=true;
        curInp.disabled=true;
        const pSpan=curCtr.querySelector(".input-prompt"),pText=pSpan?pSpan.textContent:"Input:";
        fetch("/api/input",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({id,value:v})})
            .then(()=>{
                if(curCtr&&curCtr.parentNode){
                    const cDiv=document.createElement("div");
                    cDiv.className="terminal-line";
                    cDiv.innerHTML=`<span class="input-prompt">${pText}</span> <span class="completed-input">${v}</span>`;
                    curCtr.parentNode.insertBefore(cDiv,curCtr);
                    curCtr.remove();
                    term.scrollTop=term.scrollHeight;
                }
                procInp=false;
                curId=curInp=curErr=curCtr=null;
            })
            .catch(()=>{procInp=false;if(curInp)curInp.disabled=false;});
    };
    const buildInp=(id,prompt,restore,frag,focTasks)=>{
        curId=id;
        curCtr=document.createElement("div");
        curCtr.className="input-container";
        const row=document.createElement("div");
        row.className="input-line";
        const span=document.createElement("span");
        span.className="input-prompt";
        span.innerHTML=prompt+" ";
        const inp=document.createElement("input");
        inp.className="user-input";
        inp.type="text";
        inp.value=restore;
        curInp=inp;
        inp.addEventListener("keypress",e=>{
            if(e.key!=="Enter")return;
            const val=inp.value.trim();
            if(!val){showErr("Please enter a value");inp.focus();return;}
            fetch("/api/validate",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({id,value:val})})
                .then(r=>r.json())
                .then(res=>{if(res?.valid){clrErr();submit();}else{showErr(res?.error||"Invalid value");inp.value="";inp.focus();}})
                .catch(()=>{showErr("Validation failed");inp.focus();});
        });
        inp.addEventListener("input",()=>{if(inp.value.trim()&&curErr)clrErr();});
        row.appendChild(span);row.appendChild(inp);curCtr.appendChild(row);frag.appendChild(curCtr);
        focTasks.push(()=>{inp.focus();const L=inp.value.length;try{inp.setSelectionRange(L,L);}catch{}});
    };
    const update=()=>{
        if(procInp||document.hidden)return;
        inFlight?.abort?.();
        const ctrl=new AbortController();
        inFlight=ctrl;
        fetch("/api/state",{signal:ctrl.signal})
            .then(r=>r.json())
            .then(data=>{
                if(!term||!Array.isArray(data?.output))return;
                if(rendCnt>data.output.length){term.innerHTML="";rendCnt=0;}
                if(rendCnt===data.output.length)return;
                const frag=document.createDocumentFragment(),mathNodes=[],scriptNodes=[],focTasks=[];
                let i=rendCnt,restore="";
                if(curInp&&!curInp.disabled)restore=curInp.value;
                if(curCtr)curCtr.remove();
                curInp=curId=curErr=curCtr=null;
                for(;i<data.output.length;i++){
                    const line=String(data.output[i]??"");
                    if(line.startsWith("OBJECT_STAGE:")){appendStage(JSON.parse(line.slice(13)),frag);continue;}
                    if(line.startsWith("OBJECT_CMDS:")){const p=JSON.parse(line.slice(12));window.webrustTurtle.handleCmds(p);continue;}
                    if(line.startsWith("OBJECT_GROUP:")){const p=JSON.parse(line.slice(13));window.webrustTurtle.handleGroup(p);continue;}
                    if(line.startsWith("SIMPLE_TABLE:")){
                        const html=line.substring(13),div=document.createElement("div");
                        div.className="table-container";
                        div.innerHTML=html;
                        frag.appendChild(div);
                        if(hasMath(html))mathNodes.push(div);
                        continue;
                    }
                    if(line.startsWith("INPUT_REQUEST:")){
                        const parts=line.split(":"),id=parts[1],prompt=parts.slice(2).join(":");
                        buildInp(id,prompt,restore,frag,focTasks);
                        continue;
                    }
                    const div=document.createElement("div");
                    div.className="terminal-line";
                    div.innerHTML=line;
                    frag.appendChild(div);
                    if(hasMath(line))mathNodes.push(div);
                    if(line.includes("<script>"))scriptNodes.push(div);
                }
                term.appendChild(frag);
                for(const f of focTasks)f();
                if(mathNodes.length)renderMath(mathNodes);
                if(scriptNodes.length)runScripts(scriptNodes);
                rendCnt=data.output.length;
                term.scrollTop=term.scrollHeight;
            })
            .catch(err=>{if(err?.name!=="AbortError")console.error("Error fetching state:",err);})
            .finally(()=>{inFlight=null;});
    };
    let rafSched=false;
    const sched=()=>{
        if(!rafSched&&!procInp&&!document.hidden){
            rafSched=true;
            requestAnimationFrame(()=>{rafSched=false;update();});
        }
    };
    setInterval(sched,100);
    sched();
})();