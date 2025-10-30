/* webrust/static/turtle.js */
/* global Two */
/** @ts-check */
/** @typedef {"css"|"cartesian"} CoordMode */
/** @typedef {"color"|"width"|"speed"|"fill"|"dash"|"point"|"line"|"circle"|"arc"|"ellipse"|"rectangle"|"square"|"rhombus"|"parallelogram"|"polygon"|"translate"|"rotate"|"scale"|"reflect"|"pen_move"|"set_position"|"set_pen_angle"|"adjust_pen_angle"|"style_mode"|"ease"|"reverse"|"wait"} ObjectOp */
/** @typedef {{op:ObjectOp, a?:number, b?:number, c?:number, d?:number, s?:string, points?:number[][]}} ObjectCmd */
/** @typedef {{id:string, w:number, h:number, coord?:CoordMode}} ObjectStage */
/** @typedef {{stage:string, oid:string, cmds:ObjectCmd[], w?:number, h?:number}} ObjectPacket */
/** @typedef {{gid:string, members:string[], cmds:ObjectCmd[]}} GroupPacket */
/** @typedef {{x:number, y:number}} TwoVec */
/** @typedef {{x:number, y:number, ux?:number, uy?:number}} TwoAnch */
/** @typedef {{id:string, two:any, el:HTMLElement, w:number, h:number, coord:CoordMode, objects:Map<string, any>}} StageState */
/** @typedef {{x:number, y:number, _flagVertices?:boolean}} TwoVertex */
/** @typedef {{children?:any[], stroke?:string, fill?:string, linewidth?:number, dashes?:number[], translation?:TwoVec, rotation?:number, scale?:any, _flagScale?:boolean, width?:number, height?:number, radius?:number, radiusX?:number, radiusY?:number, vertices?:TwoVertex[], noFill?:function():void}} TwoShape */
/** @typedef {function(new:TwoShape, any[], boolean, boolean):TwoShape} TwoPathConstructor */
/** @typedef {{Vector:any, Anchor:any, Path:TwoPathConstructor}} TwoTypes */
/** @typedef {{width:number, height:number, autostart:boolean, appendTo:function(HTMLElement):any, makeGroup:function():TwoShape, makeCircle:function(number,number,number):TwoShape, makeLine:function(number,number,number,number):TwoShape, makeEllipse:function(number,number,number,number):TwoShape, makeRectangle:function(number,number,number,number):TwoShape, add:function(TwoShape):void, update:function():void, renderer:{setSize:function(number,number):void}}} TwoInstance */
/** @typedef {function(new:TwoInstance, {width:number,height:number,autostart:boolean}):TwoInstance & TwoTypes} TwoConstructor */
(() => {
    "use strict";
    const TwoGlobal = typeof Two !== "undefined" ? Two : void 0;
    const Vec = (x = 0, y = 0) => { const C = TwoGlobal && TwoGlobal["Vector"]; return C ? new C(x, y) : { x, y }; };
    const Anch = (x = 0, y = 0) => { const C = TwoGlobal && TwoGlobal["Anchor"]; return C ? new C(x, y) : { x, y }; };
    const yfix = (st, y) => (st.coord === "cartesian" ? -y : y);
    const rotFix = (st, deg) => (st.coord === "cartesian" ? -deg * Math.PI / 180 : deg * Math.PI / 180);
    const STAGES = new Map();
    const ensureNSS = () => { if (document.getElementById("webrust-nss")) return; const st = document.createElement("style"); st.id = "webrust-nss"; st.textContent = ".object-stage svg path,.object-stage svg line,.object-stage svg rect,.object-stage svg circle,.object-stage svg ellipse,.object-stage svg polygon,.object-stage svg polyline{vector-effect:non-scaling-stroke}"; document.head.appendChild(st); };
    const applyTransformImmediate = (o, t) => { if (t.rotation !== undefined) o.drawGroup.rotation = t.rotation; if (t.scale) { o.drawGroup.scale = Vec(t.scale.x, t.scale.y); o.drawGroup._flagScale = true; } if (t.translation) { o.group.translation.x = t.translation.x; o.group.translation.y = t.translation.y; } };
    const getScaleValues = (o) => { let x = 1, y = 1; if (o.drawGroup.scale) { if (typeof o.drawGroup.scale === "number") { x = y = o.drawGroup.scale; } else { x = o.drawGroup.scale.x || 1; y = o.drawGroup.scale.y || 1; } } return { x, y }; };
    const Easings = (() => {
        const PI = Math.PI, linear = t => t, sineIn = t => 1 - Math.cos(t * PI / 2), sineOut = t => Math.sin(t * PI / 2), sineInOut = t => -(Math.cos(PI * t) - 1) / 2, quadIn = t => t * t, quadOut = t => 1 - (1 - t) * (1 - t), quadInOut = t => t < .5 ? 2 * t * t : 1 - Math.pow(-2 * t + 2, 2) / 2, cubicIn = t => t * t * t, cubicOut = t => 1 - Math.pow(1 - t, 3), cubicInOut = t => t < .5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2, quartIn = t => t ** 4, quartOut = t => 1 - (1 - t) ** 4, quartInOut = t => t < .5 ? 8 * t ** 4 : 1 - ((-2 * t + 2) ** 4) / 2, quintIn = t => t ** 5, quintOut = t => 1 - (1 - t) ** 5, quintInOut = t => t < .5 ? 16 * t ** 5 : 1 - ((-2 * t + 2) ** 5) / 2, expoIn = t => t === 0 ? 0 : Math.pow(2, 10 * (t - 1)), expoOut = t => t === 1 ? 1 : 1 - Math.pow(2, -10 * t), expoInOut = t => t === 0 ? 0 : t === 1 ? 1 : t < .5 ? Math.pow(2, 20 * t - 10) / 2 : (2 - Math.pow(2, -20 * t + 10)) / 2, circIn = t => 1 - Math.sqrt(1 - t * t), circOut = t => Math.sqrt(1 - (t - 1) * (t - 1)), circInOut = t => t < .5 ? (1 - Math.sqrt(1 - (2 * t) * (2 * t))) / 2 : (Math.sqrt(1 - (-2 * t + 2) * (-2 * t + 2)) + 1) / 2, backIn = (t, s = 1.70158) => t * t * ((s + 1) * t - s), backOut = (t, s = 1.70158) => { t -= 1; return t * t * ((s + 1) * t + s) + 1; }, backInOut = t => { const s = 1.70158 * 1.525; return t < .5 ? Math.pow(2 * t, 2) * ((s + 1) * 2 * t - s) / 2 : Math.pow(2 * t - 2, 2) * ((s + 1) * (2 * t - 2) + s) / 2 + 1; }, elasticIn = t => { if (t === 0 || t === 1) return t; const p = .3, s = p / 4; return -Math.pow(2, 10 * (t - 1)) * Math.sin((t - 1 - s) * 2 * PI / p); }, elasticOut = t => { if (t === 0 || t === 1) return t; const p = .3, s = p / 4; return Math.pow(2, -10 * t) * Math.sin((t - s) * 2 * PI / p) + 1; }, elasticInOut = t => { if (t === 0 || t === 1) return t; const p = .45, s = p / 4; t *= 2; if (t < 1) return -.5 * Math.pow(2, 10 * (t - 1)) * Math.sin((t - 1 - s) * 2 * PI / p); t -= 1; return .5 * Math.pow(2, -10 * t) * Math.sin((t - s) * 2 * PI / p) + 1; }, bounceOut = t => { const n1 = 7.5625, d1 = 2.75; if (t < 1 / d1) return n1 * t * t; if (t < 2 / d1) { t -= 1.5 / d1; return n1 * t * t + .75; } if (t < 2.5 / d1) { t -= 2.25 / d1; return n1 * t * t + .9375; } t -= 2.625 / d1; return n1 * t * t + .984375; }, bounceIn = t => 1 - bounceOut(1 - t), bounceInOut = t => t < .5 ? (1 - bounceOut(1 - 2 * t)) / 2 : (bounceOut(2 * t - 1) + 1) / 2;
        const table = new Map([["linear", linear], ["sinein", sineIn], ["sineout", sineOut], ["sineinout", sineInOut], ["quadin", quadIn], ["quadout", quadOut], ["quadinout", quadInOut], ["cubicin", cubicIn], ["cubicout", cubicOut], ["cubicinout", cubicInOut], ["quartin", quartIn], ["quartout", quartOut], ["quartinout", quartInOut], ["quintin", quintIn], ["quintout", quintOut], ["quintinout", quintInOut], ["expoin", expoIn], ["expoout", expoOut], ["expoinout", expoInOut], ["circin", circIn], ["circout", circOut], ["circinout", circInOut], ["backin", backIn], ["backout", backOut], ["backinout", backInOut], ["elasticin", elasticIn], ["elasticout", elasticOut], ["elasticinout", elasticInOut], ["bouncein", bounceIn], ["bounceout", bounceOut], ["bounceinout", bounceInOut]]);
        const add = (b, i, o, io) => { table.set(`${b}in`, i); table.set(`${b}out`, o); table.set(`${b}inout`, io); table.set(`${b}In`, i); table.set(`${b}Out`, o); table.set(`${b}InOut`, io); };
        add("sine", sineIn, sineOut, sineInOut); add("quad", quadIn, quadOut, quadInOut); add("cubic", cubicIn, cubicOut, cubicInOut); add("quart", quartIn, quartOut, quartInOut); add("quint", quintIn, quintOut, quintInOut); add("expo", expoIn, expoOut, expoInOut); add("circ", circIn, circOut, circInOut); add("back", backIn, backOut, backInOut); add("elastic", elasticIn, elasticOut, elasticInOut); add("bounce", bounceIn, bounceOut, bounceInOut);
        Object.freeze(table);
        const get = (n) => { if (!n) return linear; const k = String(n).trim().toLowerCase().replace(/\s+/g, "").replace(/-/g, ""); return table.get(k) || linear; };
        return { get, linear };
    })();
    const animateTransform = (st, o, target, duration, easer, done) => {
        if (duration <= 50) { applyTransformImmediate(o, target); st.two.update(); if (done) done(); return; }
        const tok = (o.animToken || 0), t0 = performance.now();
        const start = { rotation: o.drawGroup.rotation || 0, scale: o.drawGroup.scale && o.drawGroup.scale.x !== undefined ? { x: o.drawGroup.scale.x, y: o.drawGroup.scale.y } : { x: o.drawGroup.scale || 1, y: o.drawGroup.scale || 1 }, translation: { x: o.group.translation.x || 0, y: o.group.translation.y || 0 } };
        const easeFn = typeof easer === "function" ? easer : Easings.get(o.easeName);
        const step = t => {
            if ((o.animToken || 0) !== tok) return;
            const p = Math.min((t - t0) / duration, 1), e = easeFn(p);
            if (target.rotation !== undefined) o.drawGroup.rotation = start.rotation + (target.rotation - start.rotation) * e;
            if (target.scale) { o.drawGroup.scale = Vec(start.scale.x + (target.scale.x - start.scale.x) * e, start.scale.y + (target.scale.y - start.scale.y) * e); o.drawGroup._flagScale = true; }
            if (target.translation) { o.group.translation.x = start.translation.x + (target.translation.x - start.translation.x) * e; o.group.translation.y = start.translation.y + (target.translation.y - start.translation.y) * e; }
            st.two.update();
            if (p < 1) requestAnimationFrame(step); else if (done && (o.animToken || 0) === tok) done();
        };
        requestAnimationFrame(step);
    };
    const ensureStage = (id, w, h, coord) => {
        let st = STAGES.get(id);
        if (!st) {
            let host = document.getElementById("ostage-" + id);
            if (!host) { host = document.createElement("div"); host.id = "ostage-" + id; host.className = "object-stage"; (document.getElementById("terminal") || document.body).appendChild(host); } else { host.innerHTML = ""; }
            host.style.width = w + "px"; host.style.height = h + "px";
            const two = new TwoGlobal({ width: w, height: h, autostart: true }).appendTo(host);
            st = { id, two, el: host, w, h, coord: coord || "css", objects: new Map() };
            STAGES.set(id, st);
        } else {
            if (st.w !== w || st.h !== h) { st.w = w; st.h = h; st.el.style.width = w + "px"; st.el.style.height = h + "px"; st.two.width = w; st.two.height = h; st.two.renderer.setSize(w, h); }
            st.coord = coord || st.coord;
        }
        return st;
    };
    const getObj = (st, oid) => {
        let o = st.objects.get(oid);
        if (!o) {
            ensureNSS();
            const group = st.two.makeGroup(), drawGroup = st.two.makeGroup();
            group.add(drawGroup);
            if (st.coord === "cartesian") group.translation.set(st.w / 2, st.h / 2);
            o = { color: "black", fillColor: "transparent", penSize: 1, dashPattern: null, speed: 100, group, drawGroup, animToken: 0, curX: 0, curY: 0, restyleExisting: false, easeName: "linear" };
            st.two.add(group); st.objects.set(oid, o);
        }
        return o;
    };
    const restyleGroupRecursive = (gp, opts) => { const kids = (gp && gp.children) || []; for (const e of kids) { if (opts.stroke != null) e.stroke = opts.stroke; if (opts.fill != null && "fill" in e) e.fill = opts.fill; if (opts.width != null) e.linewidth = opts.width; if ("dash" in opts && "dashes" in e) e.dashes = opts.dash; if (e.children && e.children.length) restyleGroupRecursive(e, opts); } };
    const addShape = (st, o, sh) => { sh.stroke = o.color; sh.fill = o.fillColor; sh.linewidth = o.penSize; if (o.dashPattern && sh.dashes !== undefined) sh.dashes = o.dashPattern; o.drawGroup.add(sh); st.two.update(); };
    const polyFrom = (pts) => { const a = pts || []; const n = a.length; const anchors = new Array(n); for (let i = 0; i < n; i++) anchors[i] = Anch(a[i][0], a[i][1]); return new TwoGlobal.Path(anchors, true, false); };
    const arcPath = (st, cx, cy, r, deg) => { const n = Math.max(2, Math.ceil(Math.abs(deg) / 8)); const v = []; const signY = st.coord === "cartesian" ? -1 : 1; for (let i = 0; i <= n; i++) { const th = i / n * deg * Math.PI / 180; v.push(Anch(cx + r * Math.cos(th), cy + signY * r * Math.sin(th))); } const p = new TwoGlobal.Path(v, false, false); p.noFill(); return p; };
    const shapeBounds = e => {
        let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
        const tx = (e.translation && e.translation.x) || 0, ty = (e.translation && e.translation.y) || 0;
        if (e.width != null && e.height != null) { const cx = tx, cy = ty, w = e.width, h = e.height; minX = cx - w / 2; maxX = cx + w / 2; minY = cy - h / 2; maxY = cy + h / 2; }
        else if (e.radius != null) { const r = e.radius, cx = tx, cy = ty; minX = cx - r; maxX = cx + r; minY = cy - r; maxY = cy + r; }
        else if (e.radiusX != null && e.radiusY != null) { const cx = tx, cy = ty, rx = e.radiusX, ry = e.radiusY; minX = cx - rx; maxX = cx + rx; minY = cy - ry; maxY = cy + ry; }
        else if (e.vertices && e.vertices.length) { for (const v of e.vertices) { const x = (v.x || 0) + tx, y = (v.y || 0) + ty; if (x < minX) minX = x; if (x > maxX) maxX = x; if (y < minY) minY = y; if (y > maxY) maxY = y; } }
        else { minX = tx; maxX = tx; minY = ty; maxY = ty; }
        return { minX, maxX, minY, maxY };
    };
    const computeGroupBounds = g => { let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity; const kids = g.children || []; if (!kids.length) return { cx: 0, cy: 0 }; for (const e of kids) { const b = shapeBounds(e); if (b.minX < minX) minX = b.minX; if (b.maxX > maxX) maxX = b.maxX; if (b.minY < minY) minY = b.minY; if (b.maxY > maxY) maxY = b.maxY; } return { cx: (minX + maxX) / 2, cy: (minY + maxY) / 2 }; };
    const normalizePivot = o => {
        const b = computeGroupBounds(o.drawGroup), cx = b.cx, cy = b.cy;
        if (Math.abs(cx) < 1e-6 && Math.abs(cy) < 1e-6) return;
        const kids = o.drawGroup.children || [];
        for (const e of kids) {
            const isRectLike = e.width != null && e.height != null, isCircleLike = e.radius != null || (e.radiusX != null && e.radiusY != null), isPrimitive = isRectLike || isCircleLike;
            if (isPrimitive) { if (!e.translation) e.translation = Vec(0, 0); e.translation.x -= cx; e.translation.y -= cy; }
            else if (e.vertices && e.vertices.length) { for (const v of e.vertices) { v.x -= cx; v.y -= cy; } e._flagVertices = true; }
            else { if (!e.translation) e.translation = Vec(0, 0); e.translation.x -= cx; e.translation.y -= cy; }
        }
        if (!o.drawGroup.translation) o.drawGroup.translation = Vec(0, 0);
        o.drawGroup.translation.x += cx; o.drawGroup.translation.y += cy;
    };
    const handleStage = p => { ensureStage(p.id, p.w, p.h, p.coord || "css"); };
    const handleCmds = packet => {
        const prev = STAGES.get(packet.stage);
        const st = ensureStage(packet.stage, packet.w || (prev ? prev.w : 800), packet.h || (prev ? prev.h : 600), prev ? prev.coord : "css");
        const o = getObj(st, packet.oid);
        const cmds = packet.cmds || [];
        if (!cmds.length) return;
        let i = 0;
        const doneNow = () => { i++; next(); };
        const next = () => {
            if (i >= cmds.length) return;
            const cmd = cmds[i], op = cmd.op;
            if (op === "style_mode") { o.restyleExisting = (cmd.s || "") === "restyle_on"; return doneNow(); }
            if (op === "ease") { o.easeName = cmd.s || "linear"; return doneNow(); }
            if (op === "color") { o.color = cmd.s || "black"; if (o.restyleExisting) { restyleGroupRecursive(o.drawGroup, { stroke: o.color }); st.two.update(); } return doneNow(); }
            if (op === "fill") { o.fillColor = cmd.s || "transparent"; if (o.restyleExisting) { restyleGroupRecursive(o.drawGroup, { fill: o.fillColor }); st.two.update(); } return doneNow(); }
            if (op === "width") { o.penSize = cmd.a || 1; if (o.restyleExisting) { restyleGroupRecursive(o.drawGroup, { width: o.penSize }); st.two.update(); } return doneNow(); }
            if (op === "dash") { o.dashPattern = cmd.a != null && cmd.b != null ? [cmd.a, cmd.b] : null; if (o.restyleExisting) { restyleGroupRecursive(o.drawGroup, { dash: o.dashPattern }); st.two.update(); } return doneNow(); }
            if (op === "speed") { o.speed = Math.max(.1, cmd.a || 100); return doneNow(); }
            if (op === "set_position") { o.curX = cmd.a || 0; o.curY = yfix(st, cmd.b || 0); return doneNow(); }
            if (op === "point") { const x = cmd.a || 0, y = yfix(st, cmd.b || 0); const c = st.two.makeCircle(x, y, Math.max(1, o.penSize / 2)); c.fill = o.color; c.stroke = "transparent"; o.drawGroup.add(c); st.two.update(); return doneNow(); }
            if (op === "line") { const x1 = cmd.a || 0, y1 = yfix(st, cmd.b || 0), x2 = cmd.c || 0, y2 = yfix(st, cmd.d || 0); const L = st.two.makeLine(x1, y1, x2, y2); addShape(st, o, L); return doneNow(); }
            if (op === "circle") { const r = Math.abs(cmd.a || 10), init = cmd.c || 0; const C = st.two.makeCircle(o.curX || 0, o.curY || 0, r); C.rotation = rotFix(st, init); addShape(st, o, C); return doneNow(); }
            if (op === "arc") { const r = Math.abs(cmd.a || 10), sweep = cmd.b || 90, init = cmd.c || 0; const P = arcPath(st, o.curX || 0, o.curY || 0, r, sweep); P.stroke = o.color; P.linewidth = o.penSize; P.rotation = rotFix(st, init); addShape(st, o, P); return doneNow(); }
            if (op === "ellipse") { const rx = Math.abs(cmd.a || 10), ry = Math.abs(cmd.b || 10), init = cmd.c || 0; const E = st.two.makeEllipse(o.curX || 0, o.curY || 0, rx, ry); E.rotation = rotFix(st, init); addShape(st, o, E); return doneNow(); }
            if (op === "rectangle") { const w = cmd.a || 10, h = cmd.b || 10, init = cmd.c || 0; const R = st.two.makeRectangle(o.curX || 0, o.curY || 0, w, h); R.rotation = rotFix(st, init); addShape(st, o, R); return doneNow(); }
            if (op === "square") { const s = cmd.a || 10, init = cmd.b || 0; const R = st.two.makeRectangle(o.curX || 0, o.curY || 0, s, s); R.rotation = rotFix(st, init); addShape(st, o, R); return doneNow(); }
            if (op === "rhombus") { const side = cmd.a || 10, alpha = (cmd.b || 60) * Math.PI / 180, init = cmd.c || 0; const cx = o.curX || 0, cy = o.curY || 0; const ux = side, uy = 0, vx = side * Math.cos(alpha), vy = side * Math.sin(alpha); const pts = [[cx + (ux + vx) / 2, cy + (uy + vy) / 2], [cx + (ux - vx) / 2, cy + (uy - vy) / 2], [cx - (ux + vx) / 2, cy - (uy + vy) / 2], [cx - (ux - vx) / 2, cy - (uy - vy) / 2]]; const P = polyFrom(pts); P.rotation = rotFix(st, init); addShape(st, o, P); return doneNow(); }
            if (op === "parallelogram") { const Lg = cmd.a || 10, W = cmd.b || 10, alpha = (cmd.c || 60) * Math.PI / 180, init = cmd.d || 0; const sinA = Math.sin(alpha), cosA = Math.cos(alpha); const k = Math.abs(sinA) < 1e-6 ? 0 : W / 2 * (cosA / sinA); const cx = o.curX || 0, cy = o.curY || 0; const pts = [[cx - Lg / 2, cy - W / 2], [cx + Lg / 2, cy - W / 2], [cx + Lg / 2 - k, cy + W / 2], [cx - Lg / 2 - k, cy + W / 2]]; const P = polyFrom(pts); P.rotation = rotFix(st, init); addShape(st, o, P); return doneNow(); }
            if (op === "polygon") {
                const pts = cmd.points || [];
                if (pts.length > 2) {
                    const n = pts.length, anchors = new Array(n);
                    for (let j = 0; j < n; j++) { const p = pts[j]; anchors[j] = Anch(p[0], yfix(st, p[1])); }
                    const P = new TwoGlobal.Path(anchors, true, false);
                    addShape(st, o, P);
                }
                return doneNow();
            }
            if (op === "pen_move") { const nx = cmd.a || 0, ny = yfix(st, cmd.b || 0); const draw = cmd.c === 1.0; const x0 = o.curX || 0, y0 = o.curY || 0; o.curX = nx; o.curY = ny; if (!draw) { st.two.update(); return doneNow(); } const L = st.two.makeLine(x0, y0, nx, ny); addShape(st, o, L); st.two.update(); return doneNow(); }
            if (op === "translate") { const dx = cmd.a || 0, dy = cmd.b || 0; const DX = dx, DY = yfix(st, dy); const tx = (o.group.translation.x || 0) + DX, ty = (o.group.translation.y || 0) + DY; const dist = Math.hypot(DX, DY); const dur = Math.max(1, dist / Math.max(o.speed, 1) * 1000); const tok = (o.animToken || 0); animateTransform(st, o, { translation: { x: tx, y: ty } }, dur, Easings.get(o.easeName), () => { if ((o.animToken || 0) === tok) doneNow(); }); return; }
            if (op === "rotate") { normalizePivot(o); const deg = cmd.a || 0, rad = rotFix(st, deg); const target = (o.drawGroup.rotation || 0) + rad; const dur = Math.max(1, Math.abs(deg) / Math.max(o.speed, 1) * 1000); const tok = (o.animToken || 0); animateTransform(st, o, { rotation: target }, dur, Easings.get(o.easeName), () => { if ((o.animToken || 0) === tok) { o.drawGroup.rotation = target; st.two.update(); } doneNow(); }); return; }
            if (op === "scale") { normalizePivot(o); const sx = cmd.a == null ? 1 : cmd.a, sy = cmd.b == null ? sx : cmd.b; const cur = getScaleValues(o); const nx = cur.x * sx, ny = cur.y * sy; const mag = Math.max(Math.abs(sx - 1), Math.abs(sy - 1)); const dur = Math.max(1, mag / Math.max(o.speed / 100, .01) * 1000); const tok = (o.animToken || 0); animateTransform(st, o, { scale: { x: nx, y: ny } }, dur, Easings.get(o.easeName), () => { if ((o.animToken || 0) === tok) { o.drawGroup.scale = Vec(nx, ny); o.drawGroup._flagScale = true; st.two.update(); } doneNow(); }); return; }
            if (op === "reflect") { normalizePivot(o); const axis = (cmd.s || "x").toLowerCase(); const cur = o.drawGroup.scale || { x: 1, y: 1 }; const target = axis === "x" ? { x: cur.x, y: cur.y * -1 } : { x: cur.x * -1, y: cur.y }; const dur = Math.max(1, 1000 / Math.max(o.speed / 100, .01)); const tok = (o.animToken || 0); animateTransform(st, o, { scale: target }, dur, Easings.get(o.easeName), () => { if ((o.animToken || 0) === tok) { o.drawGroup.scale = Vec(target.x, target.y); o.drawGroup._flagScale = true; st.two.update(); } doneNow(); }); return; }
            if (op === "wait") { const ms = Math.max(1, cmd.a || 0); const tok = (o.animToken || 0); animateTransform(st, o, {}, ms, Easings.get(o.easeName), () => { if ((o.animToken || 0) === tok) doneNow(); }); return; }
            doneNow();
        };
        next();
    };
    const resetGroupAnimations = objs => { for (const o of objs) o.animToken = (o.animToken || 0) + 1; };
    const computeMaxDuration = (objs, calc) => { let d = 0; for (const o of objs) d = Math.max(d, calc(o)); return Math.max(d, 1); };
    const animateGroupObjects = (st, objs, tf, dur, next) => {
        let done = 0; const n = objs.length;
        for (const o of objs) {
            const target = tf(o); const tok = (o.animToken || 0);
            animateTransform(st, o, target, dur, Easings.get(o.easeName), () => {
                if ((o.animToken || 0) !== tok) return;
                if (target.rotation !== undefined) o.drawGroup.rotation = target.rotation;
                if (target.translation) { o.group.translation.x = target.translation.x; o.group.translation.y = target.translation.y; }
                if (target.scale) { o.drawGroup.scale = Vec(target.scale.x, target.scale.y); o.drawGroup._flagScale = true; }
                st.two.update();
                if (++done === n) next();
            });
        }
    };
    const handleGroup = packet => {
        const prev = STAGES.get("stage1");
        const st = prev || ensureStage("stage1", 800, 600, "css");
        const ids = Array.isArray(packet.members) ? packet.members : [];
        const objs = ids.map(id => st.objects.get(id)).filter(Boolean);
        if (!objs.length || !packet.cmds || !packet.cmds.length) return;
        const guessR = o => { const kids = (o.drawGroup && o.drawGroup.children) || []; let r = 0; for (const e of kids) { if (e.radius != null) r = Math.max(r, e.radius); else if (e.radiusX != null && e.radiusY != null) r = Math.max(r, (e.radiusX + e.radiusY) / 2); } return r > 0 ? r : null; };
        let i = 0;
        const next = () => {
            if (i >= packet.cmds.length) return;
            const cmd = packet.cmds[i++], op = cmd.op;
            if (op === "translate") {
                const dx = cmd.a || 0, dy = cmd.b || 0;
                const DX = dx, DY = yfix(st, dy);
                const dist = Math.hypot(DX, DY);
                const dur = computeMaxDuration(objs, o => dist / Math.max(o.speed || 100, 1) * 1000);
                resetGroupAnimations(objs);
                animateGroupObjects(st, objs, o => {
                    const tx = (o.group.translation.x || 0) + DX, ty = (o.group.translation.y || 0) + DY;
                    const target = { translation: { x: tx, y: ty } };
                    const R = guessR(o);
                    if (R && dist > 0) {
                        const dir = DX >= 0 ? 1 : -1;
                        const deg = -(dist / R) * (180 / Math.PI) * dir;
                        const rad = rotFix(st, deg);
                        target.rotation = (o.drawGroup.rotation || 0) + rad;
                    }
                    return target;
                }, dur, next);
                return;
            }
            if (op === "rotate") {
                const deg = cmd.a || 0, rad = rotFix(st, deg);
                const dur = computeMaxDuration(objs, o => Math.abs(deg) / Math.max(o.speed || 100, 1) * 1000);
                resetGroupAnimations(objs);
                for (const o of objs) normalizePivot(o);
                animateGroupObjects(st, objs, o => ({ rotation: (o.drawGroup.rotation || 0) + rad }), dur, next);
                return;
            }
            if (op === "scale") {
                const sx = cmd.a == null ? 1 : cmd.a, sy = cmd.b == null ? sx : cmd.b;
                const mag = Math.max(Math.abs(sx - 1), Math.abs(sy - 1));
                const dur = computeMaxDuration(objs, o => mag / Math.max((o.speed || 100) / 100, .01) * 1000);
                resetGroupAnimations(objs);
                for (const o of objs) normalizePivot(o);
                animateGroupObjects(st, objs, o => { const cur = getScaleValues(o); return { scale: { x: cur.x * sx, y: cur.y * sy } }; }, dur, next);
                return;
            }
            if (op === "reflect") {
                const axis = (cmd.s || "x").toLowerCase();
                const dur = computeMaxDuration(objs, o => 1000 / Math.max((o.speed || 100) / 100, .01));
                resetGroupAnimations(objs);
                for (const o of objs) normalizePivot(o);
                animateGroupObjects(st, objs, o => { const cur = o.drawGroup.scale || { x: 1, y: 1 }; return { scale: axis === "x" ? { x: cur.x, y: cur.y * -1 } : { x: cur.x * -1, y: cur.y } }; }, dur, next);
                return;
            }
            if (op === "wait") {
                const ms = cmd.a || 0;
                let done = 0;
                for (const o of objs) animateTransform(st, o, {}, ms, Easings.get(o.easeName), () => { if (++done === objs.length) next(); });
                return;
            }
            next();
        };
        next();
    };
    const API = { handleStage, handleCmds, handleGroup };
    Object.freeze(API);
    window.WebRustTurtle = window.webrustTurtle = API;
})();