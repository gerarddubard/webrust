/** @ts-check */
/** @typedef {"color"|"pen"|"speed"|"angle"|"pos"|"line"|"point"|"circle"|"fwd"|"penup"|"pendown"} TurtleOp */
/** @typedef {{op:TurtleOp,a?:number,b?:number,c?:number,d?:number,s?:string}} TurtleCmd */
/** @typedef {{id:string,w:number,h:number,coord?:"css"|"cartesian"}} TurtleStage */
/** @typedef {{stage:string,w?:number,h?:number,tid:string,cmds:TurtleCmd[]}} TurtlePacket */

(()=>{let currentInputId=null,currentInputElement=null,lastOutputLength=0,processingInput=false,currentErrorElement=null,currentInputContainer=null,inFlight=null;
    const hasMath=s=>/\$[^$]*\$|\\\(|\\\[|\\begin\{/.test(s||"");
    const showError=msg=>{currentErrorElement?.remove();currentErrorElement=document.createElement("div");currentErrorElement.className="error-message";currentErrorElement.textContent="❌ "+msg;currentInputContainer?.appendChild(currentErrorElement)};
    const clearError=()=>{if(currentErrorElement){currentErrorElement.remove();currentErrorElement=null}};

    const webrustTurtle=(()=>{const S=new Map();
        function ensureStage(id,w,h,coord="css"){
            let host=document.getElementById("tstage-"+id);
            if(!host){host=document.createElement("div");host.id="tstage-"+id;host.className="turtle-stage";(document.getElementById("terminal")||document.body).appendChild(host)}
            let c=host.querySelector("canvas");if(!c){c=document.createElement("canvas");host.appendChild(c)}
            const dpr=globalThis.devicePixelRatio||1;c.width=Math.floor(w*dpr);c.height=Math.floor(h*dpr);c.style.width=w+"px";c.style.height=h+"px";
            const ctx=c.getContext("2d");ctx.setTransform(dpr,0,0,dpr,0,0);
            let st=S.get(id);if(!st){st={id,w,h,ctx,coord,turtles:new Map(),last:performance.now()};S.set(id,st)}else{st.w=w;st.h=h;st.ctx=ctx;st.coord=coord}
            return st
        }
        function getT(st,tid){let t=st.turtles.get(tid);if(!t){t={x:0,y:0,ang:0,pen:true,color:"#000",penSize:1,speed:0,queue:[],cur:null};st.turtles.set(tid,t)}return t}
        const toPx=(st,x,y)=>st.coord==="cartesian"?[st.w/2+x,st.h/2-y]:[x,y];

        function step(st,t,dt){
            const ctx=st.ctx;
            if(!t.cur){
                /** @type {TurtleCmd|undefined} */ const cmd=t.queue.shift(); if(!cmd) return;
                const op=cmd["op"];
                if(op==="color"){t.color=cmd["s"]}
                else if(op==="pen"){t.penSize=cmd["a"]}
                else if(op==="speed"){t.speed=cmd["a"]}
                else if(op==="angle"){t.ang=cmd["a"]*Math.PI/180}
                else if(op==="pos"){t.x=cmd["a"];t.y=cmd["b"]}
                else if(op==="penup"){t.pen=false}
                else if(op==="pendown"){t.pen=true}
                else if(op==="line"){const p1=toPx(st,cmd["a"],cmd["b"]),p2=toPx(st,cmd["c"],cmd["d"]);ctx.lineWidth=t.penSize;ctx.strokeStyle=t.color;ctx.beginPath();ctx.moveTo(p1[0],p1[1]);ctx.lineTo(p2[0],p2[1]);ctx.stroke()}
                else if(op==="point"){const p=toPx(st,t.x,t.y);ctx.fillStyle=t.color;ctx.beginPath();ctx.arc(p[0],p[1],Math.max(1,t.penSize/2),0,Math.PI*2);ctx.fill()}
                else if(op==="circle"){const p=toPx(st,t.x,t.y);ctx.lineWidth=t.penSize;ctx.strokeStyle=t.color;ctx.beginPath();ctx.arc(p[0],p[1],Math.abs(cmd["a"]),0,Math.PI*2);ctx.stroke()}
                else if(op==="fwd"){t.cur={type:"fwd",rem:cmd["a"]}}
            }else if(t.cur.type==="fwd"){
                const d=Math.min(t.cur.rem,Math.max(1e-6,t.speed*dt)),x0=t.x,y0=t.y;
                if(st.coord==="cartesian"){t.x+=Math.cos(t.ang)*d;t.y+=Math.sin(t.ang)*d}else{t.x+=Math.cos(t.ang)*d;t.y+=-Math.sin(t.ang)*d}
                if(t.pen){const s1=toPx(st,x0,y0),s2=toPx(st,t.x,t.y);ctx.lineWidth=t.penSize;ctx.strokeStyle=t.color;ctx.beginPath();ctx.moveTo(s1[0],s1[1]);ctx.lineTo(s2[0],s2[1]);ctx.stroke()}
                t.cur.rem-=d; if(t.cur.rem<=1e-6) t.cur=null
            }
        }

        (function loop(){const now=performance.now();S.forEach(st=>{const dt=Math.min(0.05,(now-(st.last||now))/1000);st.last=now;st.turtles.forEach(t=>step(st,t,dt))});requestAnimationFrame(loop)})();
        return{
            /** @param {TurtleStage} p */ handleStage:p=>{ensureStage(p["id"],p["w"],p["h"],p["coord"]||"css")},
            /** @param {TurtlePacket} p */ handleCmds:p=>{const prev=S.get(p["stage"]);const st=ensureStage(p["stage"],p["w"]||(prev?.w||800),p["h"]||(prev?.h||600),prev?.coord||"css");const t=getT(st,p["tid"]);t.queue.push(...p["cmds"])}
        }
    })();

    function renderMathJax(c){
        const mj=window.MathJax; if(!mj){setTimeout(()=>renderMathJax(c),200);return}
        const tp=mj["typesetPromise"]; if(typeof tp==="function"){tp.call(mj,[c]).catch(e=>console.error("MathJax:",e?.message||e));return}
        const ts=mj["typeset"]; if(typeof ts==="function"){try{ts.call(mj,[c])}catch(e){console.error("MathJax:",e?.message||e)}return}
        setTimeout(()=>renderMathJax(c),200)
    }

    function updateDisplay(){
        if(processingInput||document.hidden) return;
        inFlight?.abort?.(); const ctrl=new AbortController(); inFlight=ctrl;
        fetch("/api/state",{signal:ctrl.signal}).then(r=>r.json()).then(data=>{
            const term=document.getElementById("terminal"); if(!term||!Array.isArray(data?.output)) return;
            if(data.output.length===lastOutputLength) return; lastOutputLength=data.output.length;
            const restore=(currentInputElement&&!currentInputElement.disabled)?currentInputElement.value:"";
            term.innerHTML=""; currentInputElement=null; currentInputId=null; currentErrorElement=null; currentInputContainer=null;
            let needMath=false; const frag=document.createDocumentFragment(),postTasks=[];
            for(let i=0;i<data.output.length;i++){
                const line=String(data.output[i]??"");
                if(line.startsWith("TURTLE_STAGE:")){const p=JSON.parse(line.slice(13));let d=document.getElementById("tstage-"+p["id"]);if(!d){d=document.createElement("div");d.id="tstage-"+p["id"];d.className="turtle-stage"}frag.appendChild(d);postTasks.push(()=>webrustTurtle.handleStage(p));continue}
                if(line.startsWith("TURTLE_CMDS:")){const p=JSON.parse(line.slice(12));postTasks.push(()=>webrustTurtle.handleCmds(p));continue}
                if(line.startsWith("SIMPLE_TABLE:")){const html=line.substring(13),div=document.createElement("div");div.className="table-container";div.innerHTML=html;frag.appendChild(div);if(hasMath(html))needMath=true;continue}
                if(line.startsWith("INPUT_REQUEST:")){
                    const parts=line.split(":"),id=parts[1],prompt=parts.slice(2).join(":"),next=i+1<data.output.length?String(data.output[i+1]??""):null;
                    const completed=next&&!next.startsWith("INPUT_REQUEST:")&&!next.startsWith("PROGRAM_FINISHED");
                    if(completed){const div=document.createElement("div");div.className="terminal-line";div.innerHTML=`<span class="input-prompt">${prompt}</span> <span class="completed-input">${next}</span>`;frag.appendChild(div);if(hasMath(next))needMath=true;i++;continue}
                    currentInputId=id; currentInputContainer=document.createElement("div"); currentInputContainer.className="input-container";
                    const row=document.createElement("div"); row.className="input-line";
                    const span=document.createElement("span"); span.className="input-prompt"; span.innerHTML=prompt+" ";
                    const inp=document.createElement("input"); inp.className="user-input"; inp.type="text"; inp.value=restore; currentInputElement=inp;
                    inp.addEventListener("keypress",e=>{
                        if(e.key!=="Enter")return; const v=inp.value.trim(); if(!v){showError("Please enter a value");inp.focus();return}
                        fetch("/api/validate",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({id:currentInputId,value:v})})
                            .then(r=>r.json()).then(res=>{if(res?.valid){clearError();submitInput()}else{showError(res?.error||"Invalid value");inp.value="";inp.focus()}})
                            .catch(()=>{showError("Validation failed");inp.focus()})
                    });
                    inp.addEventListener("input",()=>{if(inp.value.trim()&&currentErrorElement)clearError()});
                    row.appendChild(span); row.appendChild(inp); currentInputContainer.appendChild(row); frag.appendChild(currentInputContainer);
                    postTasks.push(()=>{inp.focus();const L=inp.value.length;try{inp.setSelectionRange(L,L)}catch{}}); continue
                }
                const prev=i>0?String(data.output[i-1]??""):null; if(prev&&prev.startsWith("INPUT_REQUEST:")) continue;
                const div=document.createElement("div"); div.className="terminal-line"; div.innerHTML=line; frag.appendChild(div);
                if(hasMath(line)) needMath=true;
                if(line.includes("<script>")&&line.includes("echarts")){
                    postTasks.push(()=>{div.querySelectorAll("script").forEach(s=>{setTimeout(()=>{try{(new Function(s.textContent))()}catch(e){console.error("Chart script:",e)}},1000)})})
                }
            }
            term.appendChild(frag); postTasks.forEach(f=>f()); if(needMath) renderMathJax(term); term.scrollTop=term.scrollHeight
        }).catch(err=>{if(err?.name!=="AbortError")console.error("Error fetching state:",err)}).finally(()=>{inFlight=null})
    }

    function submitInput(){
        if(!currentInputId||!currentInputElement||processingInput) return;
        const v=currentInputElement.value; if(!v.trim()) return; processingInput=true; currentInputElement.disabled=true;
        fetch("/api/input",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({id:currentInputId,value:v})})
            .then(()=>{processingInput=false;currentInputId=null;currentInputElement=null;currentErrorElement=null;currentInputContainer=null;lastOutputLength=0})
            .catch(err=>{console.error("Error submitting input:",err);processingInput=false;currentInputElement&&(currentInputElement.disabled=false)})
    }

    setInterval(updateDisplay,300); updateDisplay();
})();