/* webrust/static/script.js */
(() => {
    let currentInputId=null,currentInputElement=null,lastOutputLength=0,processingInput=false,currentErrorElement=null,currentInputContainer=null,inFlight=null;
    const mathRe=/\$[^$]*\$|\\\(|\\\[|\\begin\{/;
    const hasMath=s=>!!s&&mathRe.test(s);
    const showError=msg=>{if(currentErrorElement)currentErrorElement.remove();currentErrorElement=document.createElement("div");currentErrorElement.className="error-message";currentErrorElement.textContent=`❌ ${msg}`;currentInputContainer?.appendChild(currentErrorElement)};
    const clearError=()=>{if(currentErrorElement){currentErrorElement.remove();currentErrorElement=null}};

    function renderMathJax(c){
        const mj=window?.MathJax;
        if(!mj){setTimeout(()=>renderMathJax(c),200);return;}
        try{const cl=mj["typesetClear"];if(typeof cl==="function")cl.call(mj,[c]);}catch{}
        const tp=mj["typesetPromise"],ts=mj["typeset"];
        if(typeof tp==="function"){tp.call(mj,[c]).catch(e=>console.error("MathJax error:",e?.message||e));return;}
        if(typeof ts==="function"){try{ts.call(mj,[c]);}catch(e){console.error("MathJax error:",e?.message||e);}return;}
        setTimeout(()=>renderMathJax(c),200);
    }

    function updateDisplay(){
        if(processingInput)return;
        if(inFlight?.abort)inFlight.abort();
        const ctrl=new AbortController();inFlight=ctrl;
        fetch("/api/state",{signal:ctrl.signal})
            .then(r=>r.json())
            .then(data=>{
                const term=document.getElementById("terminal");
                if(!term||!Array.isArray(data?.output))return;
                if(data.output.length===lastOutputLength)return;
                lastOutputLength=data.output.length;
                const restore=(currentInputElement&&!currentInputElement.disabled)?currentInputElement.value:"";
                term.innerHTML="";currentInputElement=null;currentInputId=null;currentErrorElement=null;currentInputContainer=null;
                let needMath=false;const frag=document.createDocumentFragment(),postTasks=[];
                for(let i=0;i<data.output.length;i++){
                    const line=String(data.output[i]??"");
                    if(line.startsWith("SIMPLE_TABLE:")){
                        const html=line.substring(13),div=document.createElement("div");
                        div.className="table-container";div.innerHTML=html;frag.appendChild(div);
                        if(hasMath(html))needMath=true;continue;
                    }
                    if(line.startsWith("INPUT_REQUEST:")){
                        const parts=line.split(":"),id=parts[1],prompt=parts.slice(2).join(":"),next=i+1<data.output.length?String(data.output[i+1]??""):null;
                        const completed=next&&!next.startsWith("INPUT_REQUEST:")&&!next.startsWith("PROGRAM_FINISHED");
                        if(completed){
                            const div=document.createElement("div");
                            div.className="terminal-line";div.innerHTML=`<span class="input-prompt">${prompt}</span> <span class="completed-input">${next}</span>`;
                            frag.appendChild(div);if(hasMath(next))needMath=true;i++;
                        }else{
                            currentInputId=id;currentInputContainer=document.createElement("div");currentInputContainer.className="input-container";
                            const row=document.createElement("div");row.className="input-line";
                            const span=document.createElement("span");span.className="input-prompt";span.innerHTML=prompt+" ";
                            const inp=document.createElement("input");inp.className="user-input";inp.type="text";inp.value=restore;currentInputElement=inp;
                            inp.addEventListener("keypress",e=>{
                                if(e.key!=="Enter")return;
                                const v=inp.value.trim();
                                if(!v){showError("Please enter a value");inp.focus();return;}
                                fetch("/api/validate",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({id:currentInputId,value:v})})
                                    .then(r=>r.json())
                                    .then(res=>{if(res?.valid){clearError();submitInput();}else{showError(res?.error||"Invalid value");inp.value="";inp.focus();}})
                                    .catch(()=>{showError("Validation failed");inp.focus();});
                            });
                            inp.addEventListener("input",()=>{if(inp.value.trim()&&currentErrorElement)clearError();});
                            row.appendChild(span);row.appendChild(inp);currentInputContainer.appendChild(row);frag.appendChild(currentInputContainer);
                            postTasks.push(()=>{inp.focus();const L=inp.value.length;try{inp.setSelectionRange(L,L);}catch{}});
                        }
                        continue;
                    }
                    const prev=i>0?String(data.output[i-1]??""):null;
                    if(prev&&prev.startsWith("INPUT_REQUEST:"))continue;
                    const div=document.createElement("div");div.className="terminal-line";div.innerHTML=line;frag.appendChild(div);
                    if(hasMath(line))needMath=true;

                    // Gestion des graphiques ECharts
                    if(line.includes('<script>')&&line.includes('echarts')){
                        postTasks.push(()=>{
                            const scripts=div.querySelectorAll('script');
                            scripts.forEach(script=>{
                                setTimeout(()=>{
                                    try{
                                        eval(script.textContent);
                                    }catch(e){
                                        console.error('Chart script error:',e);
                                    }
                                },1000);
                            });
                        });
                    }
                }
                term.appendChild(frag);postTasks.forEach(f=>f());if(needMath)renderMathJax(term);term.scrollTop=term.scrollHeight;
            })
            .catch(err=>{if(err?.name!=="AbortError")console.error("Error fetching state:",err);})
            .finally(()=>{inFlight=null;});
    }

    function submitInput(){
        if(!currentInputId||!currentInputElement||processingInput)return;
        const v=currentInputElement.value;if(!v.trim())return;
        processingInput=true;currentInputElement.disabled=true;
        fetch("/api/input",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({id:currentInputId,value:v})})
            .then(()=>{processingInput=false;currentInputId=null;currentInputElement=null;currentErrorElement=null;currentInputContainer=null;lastOutputLength=0;})
            .catch(err=>{console.error("Error submitting input:",err);processingInput=false;if(currentInputElement)currentInputElement.disabled=false;});
    }

    setInterval(updateDisplay,300);updateDisplay();
})();