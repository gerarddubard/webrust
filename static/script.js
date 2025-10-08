/* webrust/static/script.js */
/** @ts-check */
/**
 * @typedef {"css"|"cartesian"} CoordMode
 * @typedef {"color"|"width"|"speed"|"fill"|"dash"|"point"|"line"|"circle"|"arc"|"ellipse"|"rectangle"|"square"|"rhombus"|"parallelogram"|"polygon"|"translate"|"rotate"|"scale"|"reflect"|"pen_move"|"set_position"|"set_pen_angle"|"adjust_pen_angle"|"style_mode"|"ease"} ObjectOp
 * @typedef {{op:ObjectOp, a?:number, b?:number, c?:number, d?:number, s?:string, points?:number[][]}} ObjectCmd
 * @typedef {{id:string, w:number, h:number, coord?:CoordMode}} ObjectStage
 * @typedef {{stage:string, oid:string, cmds:ObjectCmd[], w?:number, h?:number}} ObjectPacket
 * @typedef {{gid:string, members:string[], cmds:ObjectCmd[]}} GroupPacket
 * @typedef {{x:number, y:number}} TwoVec
 * @typedef {{x:number, y:number, ux?:number, uy?:number}} TwoAnch
 * @typedef {{id:string, two:any, el:HTMLElement, w:number, h:number, coord:CoordMode, objects:Map<string, any>}} StageState
 */
const TwoGlobal = (typeof Two !== "undefined" ? Two : undefined);
const Vec = (x = 0, y = 0) => {
    const C = TwoGlobal && TwoGlobal["Vector"];
    return C ? new C(x, y) : { x, y };
};
const Anch = (x = 0, y = 0) => {
    const C = TwoGlobal && TwoGlobal["Anchor"];
    return C ? new C(x, y) : { x, y };
};
(() => {
    let currentInputId = null;
    let currentInputElement = null;
    let lastOutputLength = 0;
    let processingInput = false;
    let currentErrorElement = null;
    let currentInputContainer = null;
    let inFlight = null;
    const hasMath = (s) => /\$[^$]*\$|\\\(|\\\[|\\begin\{/.test(s || "");
    const showError = (msg) => {
        currentErrorElement?.remove();
        currentErrorElement = document.createElement("div");
        currentErrorElement.className = "error-message";
        currentErrorElement.textContent = "❌ " + msg;
        currentInputContainer?.appendChild(currentErrorElement);
    };
    const clearError = () => {
        if (currentErrorElement) {
            currentErrorElement.remove();
            currentErrorElement = null;
        }
    };
    const webrustObjects = (() => {
        const STAGES = new Map();
        const ensureNSS = () => {
            if (document.getElementById("webrust-nss")) return;
            const st = document.createElement("style");
            st.id = "webrust-nss";
            st.textContent = `.object-stage svg path,.object-stage svg line,.object-stage svg rect,.object-stage svg circle,.object-stage svg ellipse,.object-stage svg polygon,.object-stage svg polyline{vector-effect:non-scaling-stroke}`;
            document.head.appendChild(st);
        };
        const toPx = (st, x, y) => (st.coord === "cartesian" ? [x, -y] : [x, y]);
        const applyTransformImmediate = (obj, t) => {
            if (t.rotation !== undefined) obj.drawGroup.rotation = t.rotation;
            if (t.scale) {
                obj.drawGroup.scale = Vec(t.scale.x, t.scale.y);
                obj.drawGroup._flagScale = true;
            }
            if (t.translation) {
                obj.group.translation.x = t.translation.x;
                obj.group.translation.y = t.translation.y;
            }
        };
        const getScaleValues = (o) => {
            let curX = 1, curY = 1;
            if (o.drawGroup.scale) {
                if (typeof o.drawGroup.scale === "number") {
                    curX = curY = o.drawGroup.scale;
                } else {
                    curX = o.drawGroup.scale.x || 1;
                    curY = o.drawGroup.scale.y || 1;
                }
            }
            return { x: curX, y: curY };
        };
        const Easings = (() => {
            const PI = Math.PI;
            const linear = (t) => t;
            const sineIn = (t) => 1 - Math.cos((t * PI) / 2);
            const sineOut = (t) => Math.sin((t * PI) / 2);
            const sineInOut = (t) => -(Math.cos(PI * t) - 1) / 2;
            const quadIn = (t) => t * t;
            const quadOut = (t) => 1 - (1 - t) * (1 - t);
            const quadInOut = (t) => (t < 0.5) ? (2 * t * t) : (1 - Math.pow(-2 * t + 2, 2) / 2);
            const cubicIn = (t) => t * t * t;
            const cubicOut = (t) => 1 - Math.pow(1 - t, 3);
            const cubicInOut = (t) => (t < 0.5) ? (4 * t * t * t) : (1 - Math.pow(-2 * t + 2, 3) / 2);
            const quartIn = (t) => t ** 4;
            const quartOut = (t) => 1 - (1 - t) ** 4;
            const quartInOut = (t) => (t < 0.5) ? (8 * t ** 4) : (1 - ((-2 * t + 2) ** 4) / 2);
            const quintIn = (t) => t ** 5;
            const quintOut = (t) => 1 - (1 - t) ** 5;
            const quintInOut = (t) => (t < 0.5) ? (16 * t ** 5) : (1 - ((-2 * t + 2) ** 5) / 2);
            const expoIn = (t) => (t === 0 ? 0 : Math.pow(2, 10 * (t - 1)));
            const expoOut = (t) => (t === 1 ? 1 : 1 - Math.pow(2, -10 * t));
            const expoInOut = (t) => (t === 0 ? 0 : t === 1 ? 1 :
                t < 0.5 ? Math.pow(2, 20 * t - 10) / 2
                    : (2 - Math.pow(2, -20 * t + 10)) / 2);
            const circIn = (t) => 1 - Math.sqrt(1 - t * t);
            const circOut = (t) => Math.sqrt(1 - (t - 1) * (t - 1));
            const circInOut = (t) => (t < 0.5)
                ? (1 - Math.sqrt(1 - (2 * t) * (2 * t))) / 2
                : (Math.sqrt(1 - (-2 * t + 2) * (-2 * t + 2)) + 1) / 2;
            const backIn = (t, s = 1.70158) => t * t * ((s + 1) * t - s);
            const backOut = (t, s = 1.70158) => { t -= 1; return t * t * ((s + 1) * t + s) + 1; };
            const backInOut = (t) => {
                const s = 1.70158 * 1.525;
                return (t < 0.5)
                    ? (Math.pow(2 * t, 2) * ((s + 1) * 2 * t - s)) / 2
                    : (Math.pow(2 * t - 2, 2) * ((s + 1) * (2 * t - 2) + s) + 2) / 2;
            };
            const elasticIn = (t) => {
                if (t === 0 || t === 1) return t;
                const p = 0.3, s = p / 4;
                return -Math.pow(2, 10 * (t - 1)) * Math.sin((t - 1 - s) * (2 * PI) / p);
            };
            const elasticOut = (t) => {
                if (t === 0 || t === 1) return t;
                const p = 0.3, s = p / 4;
                return Math.pow(2, -10 * t) * Math.sin((t - s) * (2 * PI) / p) + 1;
            };
            const elasticInOut = (t) => {
                if (t === 0 || t === 1) return t;
                const p = 0.45, s = p / 4;
                t *= 2;
                if (t < 1) return -0.5 * Math.pow(2, 10 * (t - 1)) * Math.sin((t - 1 - s) * (2 * PI) / p);
                t -= 1;
                return 0.5 * Math.pow(2, -10 * t) * Math.sin((t - s) * (2 * PI) / p) + 1;
            };
            const bounceOut = (t) => {
                const n1 = 7.5625, d1 = 2.75;
                if (t < 1 / d1) return n1 * t * t;
                if (t < 2 / d1) { t -= 1.5 / d1; return n1 * t * t + 0.75; }
                if (t < 2.5 / d1) { t -= 2.25 / d1; return n1 * t * t + 0.9375; }
                t -= 2.625 / d1; return n1 * t * t + 0.984375;
            };
            const bounceIn = (t) => 1 - bounceOut(1 - t);
            const bounceInOut = (t) => (t < 0.5)
                ? (1 - bounceOut(1 - 2 * t)) / 2
                : (bounceOut(2 * t - 1) + 1) / 2;
            const table = new Map([
                ["linear", linear],
                ["sinein", sineIn], ["sineout", sineOut], ["sineinout", sineInOut],
                ["quadin", quadIn], ["quadout", quadOut], ["quadinout", quadInOut],
                ["cubicin", cubicIn], ["cubicout", cubicOut], ["cubicinout", cubicInOut],
                ["quartin", quartIn], ["quartout", quartOut], ["quartinout", quartInOut],
                ["quintin", quintIn], ["quintout", quintOut], ["quintinout", quintInOut],
                ["expoin", expoIn], ["expoout", expoOut], ["expoinout", expoInOut],
                ["circin", circIn], ["circout", circOut], ["circinout", circInOut],
                ["backin", backIn], ["backout", backOut], ["backinout", backInOut],
                ["elasticin", elasticIn], ["elasticout", elasticOut], ["elasticinout", elasticInOut],
                ["bouncein", bounceIn], ["bounceout", bounceOut], ["bounceinout", bounceInOut],
            ]);
            const addAliases = (base, fnIn, fnOut, fnInOut) => {
                table.set(`${base}in`, fnIn);
                table.set(`${base}out`, fnOut);
                table.set(`${base}inout`, fnInOut);
                table.set(`${base}In`, fnIn);
                table.set(`${base}Out`, fnOut);
                table.set(`${base}InOut`, fnInOut);
            };
            addAliases("sine", sineIn, sineOut, sineInOut);
            addAliases("quad", quadIn, quadOut, quadInOut);
            addAliases("cubic", cubicIn, cubicOut, cubicInOut);
            addAliases("quart", quartIn, quartOut, quartInOut);
            addAliases("quint", quintIn, quintOut, quintInOut);
            addAliases("expo", expoIn, expoOut, expoInOut);
            addAliases("circ", circIn, circOut, circInOut);
            addAliases("back", backIn, backOut, backInOut);
            addAliases("elastic", elasticIn, elasticOut, elasticInOut);
            addAliases("bounce", bounceIn, bounceOut, bounceInOut);
            const get = (name) => {
                if (!name) return linear;
                const k = String(name).trim().toLowerCase().replace(/\s+/g, '').replace(/-/g,'');
                return table.get(k) || linear;
            };
            return { get, linear };
        })();
        const animateTransform = (st, obj, target, duration, easer, done) => {
            if (duration <= 50) {
                applyTransformImmediate(obj, target);
                st.two.update();
                if (done) done();
                return;
            }
            const myToken = (obj.animToken || 0);
            const t0 = performance.now();
            const start = {
                rotation: obj.drawGroup.rotation || 0,
                scale: obj.drawGroup.scale && obj.drawGroup.scale.x !== undefined
                    ? { x: obj.drawGroup.scale.x, y: obj.drawGroup.scale.y }
                    : { x: obj.drawGroup.scale || 1, y: obj.drawGroup.scale || 1 },
                translation: { x: obj.group.translation.x || 0, y: obj.group.translation.y || 0 }
            };
            const easeFn = typeof easer === "function" ? easer : Easings.get(obj.easeName);
            const step = (t) => {
                if ((obj.animToken || 0) !== myToken) return;
                const p = Math.min((t - t0) / duration, 1);
                const e = easeFn(p);
                if (target.rotation !== undefined) {
                    obj.drawGroup.rotation = start.rotation + (target.rotation - start.rotation) * e;
                }
                if (target.scale) {
                    obj.drawGroup.scale = Vec(
                        start.scale.x + (target.scale.x - start.scale.x) * e,
                        start.scale.y + (target.scale.y - start.scale.y) * e
                    );
                    obj.drawGroup._flagScale = true;
                }
                if (target.translation) {
                    obj.group.translation.x = start.translation.x + (target.translation.x - start.translation.x) * e;
                    obj.group.translation.y = start.translation.y + (target.translation.y - start.translation.y) * e;
                }
                st.two.update();
                if (p < 1) requestAnimationFrame(step);
                else if (done && (obj.animToken || 0) === myToken) done();
            };
            requestAnimationFrame(step);
        };
        const ensureStage = (id, w, h, coord) => {
            let st = STAGES.get(id);
            if (!st) {
                let host = document.getElementById("ostage-" + id);
                if (!host) {
                    host = document.createElement("div");
                    host.id = "ostage-" + id;
                    host.className = "object-stage";
                    (document.getElementById("terminal") || document.body).appendChild(host);
                } else {
                    host.innerHTML = "";
                }
                host.style.width = w + "px";
                host.style.height = h + "px";
                const two = new TwoGlobal({ width: w, height: h, autostart: true }).appendTo(host);
                st = { id, two, el: host, w, h, coord: coord || "css", objects: new Map() };
                STAGES.set(id, st);
            } else {
                if (st.w !== w || st.h !== h) {
                    st.w = w; st.h = h;
                    st.el.style.width = w + "px";
                    st.el.style.height = h + "px";
                    st.two.width = w;
                    st.two.height = h;
                    st.two.renderer.setSize(w, h);
                }
                st.coord = coord || st.coord;
            }
            return st;
        };
        const getObj = (st, oid) => {
            let o = st.objects.get(oid);
            if (!o) {
                ensureNSS();
                const group = st.two.makeGroup();
                const drawGroup = st.two.makeGroup();
                group.add(drawGroup);
                if (st.coord === "cartesian") group.translation.set(st.w / 2, st.h / 2);
                o = {
                    color: "black",
                    fillColor: "transparent",
                    penSize: 1,
                    dashPattern: null,
                    speed: 100,
                    group, drawGroup,
                    animationQueue: [], isAnimating: false, animToken: 0,
                    curX: 0, curY: 0,
                    restyleExisting: false,
                    easeName: "linear",
                };
                st.two.add(group);
                st.objects.set(oid, o);
            }
            return o;
        };
        const restyleGroupRecursive = (gp, opts) => {
            const kids = (gp && gp.children) || [];
            for (const e of kids) {
                if (opts.stroke != null) e.stroke = opts.stroke;
                if (opts.fill != null && "fill" in e) e.fill = opts.fill;
                if (opts.width != null) e.linewidth = opts.width;
                if ("dash" in opts && "dashes" in e) e.dashes = opts.dash;
                if (e.children && e.children.length) restyleGroupRecursive(e, opts);
            }
        };
        const addShape = (st, o, sh) => {
            sh.stroke = o.color;
            sh.fill = o.fillColor;
            sh.linewidth = o.penSize;
            if (o.dashPattern && sh.dashes !== undefined) sh.dashes = o.dashPattern;
            o.drawGroup.add(sh);
            st.two.update();
        };
        const polyFrom = (pts) => {
            const anchors = (pts || []).map((p) => Anch(p[0], p[1]));
            return new TwoGlobal.Path(anchors, true, false);
        };
        const arcPath = (st, cx, cy, r, deg) => {
            const n = Math.max(2, Math.ceil(Math.abs(deg) / 8));
            const v = [];
            const signY = st.coord === "cartesian" ? -1 : 1;
            for (let i = 0; i <= n; i++) {
                const th = (i / n) * deg * Math.PI / 180;
                v.push(Anch(cx + r * Math.cos(th), cy + signY * r * Math.sin(th)));
            }
            const p = new TwoGlobal.Path(v, false, false);
            p.noFill();
            return p;
        };
        const degToRadCart = (st, deg) => {
            const rad = (deg * Math.PI) / 180;
            return st.coord === "cartesian" ? -rad : rad;
        };
        const shapeBounds = (e) => {
            let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
            const tx = (e.translation && e.translation.x) || 0;
            const ty = (e.translation && e.translation.y) || 0;
            if (e.width != null && e.height != null) {
                const cx = tx, cy = ty, w = e.width, h = e.height;
                minX = cx - w / 2; maxX = cx + w / 2; minY = cy - h / 2; maxY = cy + h / 2;
            } else if (e.radius != null) {
                const r = e.radius, cx = tx, cy = ty;
                minX = cx - r; maxX = cx + r; minY = cy - r; maxY = cy + r;
            } else if (e.radiusX != null && e.radiusY != null) {
                const cx = tx, cy = ty, rx = e.radiusX, ry = e.radiusY;
                minX = cx - rx; maxX = cx + rx; minY = cy - ry; maxY = cy + ry;
            } else if (e.vertices && e.vertices.length) {
                for (const v of e.vertices) {
                    const x = (v.x || 0) + tx, y = (v.y || 0) + ty;
                    if (x < minX) minX = x;
                    if (x > maxX) maxX = x;
                    if (y < minY) minY = y;
                    if (y > maxY) maxY = y;
                }
            } else {
                minX = tx; maxX = tx; minY = ty; maxY = ty;
            }
            return { minX, maxX, minY, maxY };
        };
        const computeGroupBounds = (g) => {
            let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
            const kids = g.children || [];
            if (!kids.length) return { cx: 0, cy: 0 };
            for (const e of kids) {
                const b = shapeBounds(e);
                if (b.minX < minX) minX = b.minX;
                if (b.maxX > maxX) maxX = b.maxX;
                if (b.minY < minY) minY = b.minY;
                if (b.maxY > maxY) maxY = b.maxY;
            }
            return { cx: (minX + maxX) / 2, cy: (minY + maxY) / 2 };
        };
        const normalizePivot = (o) => {
            const b = computeGroupBounds(o.drawGroup);
            const cx = b.cx, cy = b.cy;
            if (Math.abs(cx) < 1e-6 && Math.abs(cy) < 1e-6) return;
            const kids = o.drawGroup.children || [];
            for (const e of kids) {
                const isRectLike = e.width != null && e.height != null;
                const isCircleLike = e.radius != null || (e.radiusX != null && e.radiusY != null);
                const isPrimitive = isRectLike || isCircleLike;
                if (isPrimitive) {
                    if (!e.translation) e.translation = Vec(0, 0);
                    e.translation.x -= cx; e.translation.y -= cy;
                } else if (e.vertices && e.vertices.length) {
                    for (const v of e.vertices) { v.x -= cx; v.y -= cy; }
                    e._flagVertices = true;
                } else {
                    if (!e.translation) e.translation = Vec(0, 0);
                    e.translation.x -= cx; e.translation.y -= cy;
                }
            }
            if (!o.drawGroup.translation) o.drawGroup.translation = Vec(0, 0);
            o.drawGroup.translation.x += cx; o.drawGroup.translation.y += cy;
        };
        const handleStage = (p) => {
            ensureStage(p.id, p.w, p.h, p.coord || "css");
        };
        const handleCmds = (packet) => {
            const prev = STAGES.get(packet.stage);
            const st = ensureStage(
                packet.stage,
                packet.w || (prev ? prev.w : 800),
                packet.h || (prev ? prev.h : 600),
                prev ? prev.coord : "css"
            );
            const o = getObj(st, packet.oid);
            const cmds = packet.cmds || [];
            let i = 0;
            const doneNow = () => { i++; next(); };
            const next = () => {
                if (i >= cmds.length) return;
                const cmd = cmds[i];
                const op = cmd.op;
                if (op === "style_mode") { o.restyleExisting = (cmd.s || "") === "restyle_on"; return doneNow(); }
                if (op === "ease") { o.easeName = cmd.s || "linear"; return doneNow(); }
                if (op === "color") { o.color = cmd.s || "black"; if (o.restyleExisting) { restyleGroupRecursive(o.drawGroup, { stroke: o.color }); st.two.update(); } return doneNow(); }
                if (op === "fill") { o.fillColor = cmd.s || "transparent"; if (o.restyleExisting) { restyleGroupRecursive(o.drawGroup, { fill: o.fillColor }); st.two.update(); } return doneNow(); }
                if (op === "width") { o.penSize = cmd.a || 1; if (o.restyleExisting) { restyleGroupRecursive(o.drawGroup, { width: o.penSize }); st.two.update(); } return doneNow(); }
                if (op === "dash") { o.dashPattern = cmd.a != null && cmd.b != null ? [cmd.a, cmd.b] : null; if (o.restyleExisting) { restyleGroupRecursive(o.drawGroup, { dash: o.dashPattern }); st.two.update(); } return doneNow(); }
                if (op === "speed") { o.speed = Math.max(0.1, cmd.a || 100); return doneNow(); }
                if (op === "set_position") { const [x, y] = toPx(st, cmd.a || 0, cmd.b || 0); o.curX = x; o.curY = y; return doneNow(); }
                if (op === "point") {
                    const [x, y] = toPx(st, cmd.a || 0, cmd.b || 0);
                    const c = st.two.makeCircle(x, y, Math.max(1, o.penSize / 2));
                    c.fill = o.color; c.stroke = "transparent";
                    o.drawGroup.add(c); st.two.update();
                    return doneNow();
                }
                if (op === "line") {
                    const p1 = toPx(st, cmd.a || 0, cmd.b || 0), p2 = toPx(st, cmd.c || 0, cmd.d || 0);
                    const L = st.two.makeLine(p1[0], p1[1], p2[0], p2[1]);
                    addShape(st, o, L);
                    return doneNow();
                }
                if (op === "circle") {
                    const r = Math.abs(cmd.a || 10), init = cmd.c || 0;
                    const C = st.two.makeCircle(o.curX || 0, o.curY || 0, r);
                    C.rotation = degToRadCart(st, init);
                    addShape(st, o, C);
                    return doneNow();
                }
                if (op === "arc") {
                    const r = Math.abs(cmd.a || 10), sweep = cmd.b || 90, init = cmd.c || 0;
                    const P = arcPath(st, o.curX || 0, o.curY || 0, r, sweep);
                    P.stroke = o.color; P.linewidth = o.penSize;
                    P.rotation = degToRadCart(st, init);
                    addShape(st, o, P);
                    return doneNow();
                }
                if (op === "ellipse") {
                    const rx = Math.abs(cmd.a || 10), ry = Math.abs(cmd.b || 10), init = cmd.c || 0;
                    const E = st.two.makeEllipse(o.curX || 0, o.curY || 0, rx, ry);
                    E.rotation = degToRadCart(st, init);
                    addShape(st, o, E);
                    return doneNow();
                }
                if (op === "rectangle") {
                    const w = cmd.a || 10, h = cmd.b || 10, init = cmd.c || 0;
                    const R = st.two.makeRectangle(o.curX || 0, o.curY || 0, w, h);
                    R.rotation = degToRadCart(st, init);
                    addShape(st, o, R);
                    return doneNow();
                }
                if (op === "square") {
                    const s = cmd.a || 10, init = cmd.b || 0;
                    const R = st.two.makeRectangle(o.curX || 0, o.curY || 0, s, s);
                    R.rotation = degToRadCart(st, init);
                    addShape(st, o, R);
                    return doneNow();
                }
                if (op === "rhombus") {
                    const side = cmd.a || 10, alpha = (cmd.b || 60) * Math.PI / 180, init = cmd.c || 0;
                    const cx = o.curX || 0, cy = o.curY || 0;
                    const ux = side, uy = 0, vx = side * Math.cos(alpha), vy = side * Math.sin(alpha);
                    const pts = [
                        [cx + (ux + vx) / 2, cy + (uy + vy) / 2],
                        [cx + (ux - vx) / 2, cy + (uy - vy) / 2],
                        [cx - (ux + vx) / 2, cy - (uy + vy) / 2],
                        [cx - (ux - vx) / 2, cy - (uy - vy) / 2]
                    ];
                    const P = polyFrom(pts);
                    P.rotation = degToRadCart(st, init);
                    addShape(st, o, P);
                    return doneNow();
                }
                if (op === "parallelogram") {
                    const Lg = cmd.a || 10, W = cmd.b || 10, alpha = (cmd.c || 60) * Math.PI / 180, init = cmd.d || 0;
                    const sinA = Math.sin(alpha), cosA = Math.cos(alpha);
                    const k = Math.abs(sinA) < 1e-6 ? 0 : (W / 2) * (cosA / sinA);
                    const cx = o.curX || 0, cy = o.curY || 0;
                    const pts = [
                        [cx - Lg / 2, cy - W / 2],
                        [cx + Lg / 2, cy - W / 2],
                        [cx + Lg / 2 - k, cy + W / 2],
                        [cx - Lg / 2 - k, cy + W / 2]
                    ];
                    const P = polyFrom(pts);
                    P.rotation = degToRadCart(st, init);
                    addShape(st, o, P);
                    return doneNow();
                }
                if (op === "polygon") {
                    const pts = (cmd.points || []).map(p => toPx(st, p[0], p[1]));
                    if (pts.length > 2) {
                        const P = polyFrom(pts);
                        addShape(st, o, P);
                    }
                    return doneNow();
                }
                if (op === "pen_move") {
                    const [nx, ny] = toPx(st, cmd.a || 0, cmd.b || 0);
                    const draw = cmd.c === 1.0;
                    const x0 = o.curX || 0, y0 = o.curY || 0;
                    o.curX = nx; o.curY = ny;
                    if (!draw) { st.two.update(); return doneNow(); }
                    const L = st.two.makeLine(x0, y0, nx, ny);
                    addShape(st, o, L);
                    st.two.update();
                    return doneNow();
                }
                if (op === "translate") {
                    const dx = cmd.a || 0, dy = cmd.b || 0;
                    const [DX, DY] = st.coord === "cartesian" ? [dx, -dy] : [dx, dy];
                    const tx = (o.group.translation.x || 0) + DX, ty = (o.group.translation.y || 0) + DY;
                    const dist = Math.hypot(DX, DY);
                    const dur = Math.max(1, (dist / Math.max(o.speed, 1)) * 1000);
                    const tok = (o.animToken || 0);
                    animateTransform(st, o, { translation: { x: tx, y: ty } }, dur, Easings.get(o.easeName), () => {
                        if ((o.animToken || 0) === tok) doneNow();
                    });
                    return;
                }
                if (op === "rotate") {
                    normalizePivot(o);
                    const deg = cmd.a || 0, rad = degToRadCart(st, deg);
                    const target = (o.drawGroup.rotation || 0) + rad;
                    const dur = Math.max(1, (Math.abs(deg) / Math.max(o.speed, 1)) * 1000);
                    const tok = (o.animToken || 0);
                    animateTransform(st, o, { rotation: target }, dur, Easings.get(o.easeName), () => {
                        if ((o.animToken || 0) === tok) { o.drawGroup.rotation = target; st.two.update(); }
                        doneNow();
                    });
                    return;
                }
                if (op === "scale") {
                    normalizePivot(o);
                    const sx = cmd.a == null ? 1 : cmd.a, sy = cmd.b == null ? sx : cmd.b;
                    const cur = getScaleValues(o);
                    const nx = cur.x * sx, ny = cur.y * sy;
                    const mag = Math.max(Math.abs(sx - 1), Math.abs(sy - 1));
                    const dur = Math.max(1, (mag / Math.max(o.speed / 100, 0.01)) * 1000);
                    const tok = (o.animToken || 0);
                    animateTransform(st, o, { scale: { x: nx, y: ny } }, dur, Easings.get(o.easeName), () => {
                        if ((o.animToken || 0) === tok) { o.drawGroup.scale = Vec(nx, ny); o.drawGroup._flagScale = true; st.two.update(); }
                        doneNow();
                    });
                    return;
                }
                if (op === "reflect") {
                    normalizePivot(o);
                    const axis = (cmd.s || "x").toLowerCase();
                    const cur = o.drawGroup.scale || { x: 1, y: 1 };
                    const target = axis === "x" ? { x: cur.x, y: cur.y * -1 } : { x: cur.x * -1, y: cur.y };
                    const dur = Math.max(1, 1000 / Math.max(o.speed / 100, 0.01));
                    const tok = (o.animToken || 0);
                    animateTransform(st, o, { scale: target }, dur, Easings.get(o.easeName), () => {
                        if ((o.animToken || 0) === tok) { o.drawGroup.scale = Vec(target.x, target.y); o.drawGroup._flagScale = true; st.two.update(); }
                        doneNow();
                    });
                    return;
                }
                if (op === "wait") {
                    const ms = Math.max(1, cmd.a || 0);
                    const tok = (o.animToken || 0);
                    animateTransform(st, o, {}, ms, Easings.get(o.easeName), () => {
                        if ((o.animToken || 0) === tok) doneNow();
                    });
                    return;
                }
                doneNow();
            };
            next();
        };
        const resetGroupAnimations = (objects) => {
            for (const o of objects) {
                o.animationQueue = [];
                o.isAnimating = false;
                o.animToken = (o.animToken || 0) + 1;
            }
        };
        const computeMaxDuration = (objects, calculator) => {
            let dur = 0;
            for (const o of objects) dur = Math.max(dur, calculator(o));
            return Math.max(dur, 1);
        };
        const animateGroupObjects = (st, groupObjs, targetFactory, dur, next) => {
            let done = 0;
            const count = groupObjs.length;
            for (const o of groupObjs) {
                const target = targetFactory(o);
                const myToken = (o.animToken || 0);
                animateTransform(st, o, target, dur, Easings.get(o.easeName), () => {
                    if ((o.animToken || 0) !== myToken) return;
                    if (target.rotation !== undefined) o.drawGroup.rotation = target.rotation;
                    if (target.translation) {
                        o.group.translation.x = target.translation.x;
                        o.group.translation.y = target.translation.y;
                    }
                    if (target.scale) {
                        o.drawGroup.scale = Vec(target.scale.x, target.scale.y);
                        o.drawGroup._flagScale = true;
                    }
                    st.two.update();
                    if (++done === count) next();
                });
            }
        };
        const handleGroup = (packet) => {
            const prev = STAGES.get("stage1");
            const st = prev || ensureStage("stage1", 800, 600, "css");
            const ids = Array.isArray(packet.members) ? packet.members : [];
            const groupObjs = ids.map(id => st.objects.get(id)).filter(Boolean);
            if (!groupObjs.length || !packet.cmds || !packet.cmds.length) return;
            const guessRadius = (o) => {
                const kids = (o.drawGroup && o.drawGroup.children) || [];
                let r = 0;
                for (const e of kids) {
                    if (e.radius != null) r = Math.max(r, e.radius);
                    else if (e.radiusX != null && e.radiusY != null) r = Math.max(r, (e.radiusX + e.radiusY) / 2);
                }
                return r > 0 ? r : null;
            };
            let i = 0;
            const next = () => {
                if (i >= packet.cmds.length) return;
                const cmd = packet.cmds[i++];
                const op = cmd.op;
                if (op === "translate") {
                    const dx = cmd.a || 0, dy = cmd.b || 0;
                    const [DX, DY] = st.coord === "cartesian" ? [dx, -dy] : [dx, dy];
                    const dist = Math.hypot(DX, DY);
                    const dur = computeMaxDuration(groupObjs, (o) => (dist / Math.max(o.speed || 100, 1)) * 1000);
                    resetGroupAnimations(groupObjs);
                    animateGroupObjects(st, groupObjs, (o) => {
                        const tx = (o.group.translation.x || 0) + DX;
                        const ty = (o.group.translation.y || 0) + DY;
                        const target = { translation: { x: tx, y: ty } };
                        const R = guessRadius(o);
                        if (R && dist > 0) {
                            const dir = (DX >= 0 ? 1 : -1);
                            const deg = -(dist / R) * (180 / Math.PI) * dir;
                            const rad = degToRadCart(st, deg);
                            target.rotation = (o.drawGroup.rotation || 0) + rad;
                        }
                        return target;
                    }, dur, next);
                    return;
                }
                if (op === "rotate") {
                    const deg = cmd.a || 0, rad = degToRadCart(st, deg);
                    const dur = computeMaxDuration(groupObjs, (o) => (Math.abs(deg) / Math.max(o.speed || 100, 1)) * 1000);
                    resetGroupAnimations(groupObjs);
                    for (const o of groupObjs) normalizePivot(o);
                    animateGroupObjects(st, groupObjs, (o) => ({ rotation: (o.drawGroup.rotation || 0) + rad }), dur, next);
                    return;
                }
                if (op === "scale") {
                    const sx = cmd.a == null ? 1 : cmd.a, sy = cmd.b == null ? sx : cmd.b;
                    const mag = Math.max(Math.abs(sx - 1), Math.abs(sy - 1));
                    const dur = computeMaxDuration(groupObjs, (o) => (mag / Math.max((o.speed || 100) / 100, 0.01)) * 1000);
                    resetGroupAnimations(groupObjs);
                    for (const o of groupObjs) normalizePivot(o);
                    animateGroupObjects(st, groupObjs, (o) => {
                        const cur = getScaleValues(o);
                        return { scale: { x: cur.x * sx, y: cur.y * sy } };
                    }, dur, next);
                    return;
                }
                if (op === "reflect") {
                    const axis = (cmd.s || "x").toLowerCase();
                    const dur = computeMaxDuration(groupObjs, (o) => 1000 / Math.max((o.speed || 100) / 100, 0.01));
                    resetGroupAnimations(groupObjs);
                    for (const o of groupObjs) normalizePivot(o);
                    animateGroupObjects(st, groupObjs, (o) => {
                        const cur = o.drawGroup.scale || { x: 1, y: 1 };
                        return { scale: axis === "x" ? { x: cur.x, y: cur.y * -1 } : { x: cur.x * -1, y: cur.y } };
                    }, dur, next);
                    return;
                }
                if (op === "wait") {
                    const ms = cmd.a || 0;
                    let done = 0;
                    for (const o of groupObjs) {
                        animateTransform(st, o, {}, ms, Easings.get(o.easeName), () => {
                            if (++done === groupObjs.length) next();
                        });
                    }
                    return;
                }
                next();
            };
            next();
        };
        return { handleStage, handleCmds, handleGroup };
    })();
    const renderMathJax = (c) => {
        const mj = window.MathJax;
        if (!mj) { setTimeout(() => renderMathJax(c), 200); return; }
        const tp = mj["typesetPromise"];
        if (typeof tp === "function") { tp.call(mj, [c]).catch(() => {}); return; }
        const ts = mj["typeset"];
        if (typeof ts === "function") { try { ts.call(mj, [c]); } catch {} return; }
        setTimeout(() => renderMathJax(c), 200);
    };
    const runInlineScripts = (div) => {
        const scripts = div.querySelectorAll("script");
        scripts.forEach((s) => {
            setTimeout(() => {
                try { (new Function(String(s.textContent)))(); } catch {}
            }, 1000);
        });
    };
    const updateDisplay = () => {
        if (processingInput || document.hidden) return;
        inFlight?.abort?.();
        const ctrl = new AbortController();
        inFlight = ctrl;
        fetch("/api/state", { signal: ctrl.signal })
            .then((r) => r.json())
            .then((data) => {
                const term = document.getElementById("terminal");
                if (!term || !Array.isArray(data?.output)) return;
                if (data.output.length === lastOutputLength) return;
                lastOutputLength = data.output.length;
                const restore = currentInputElement && !currentInputElement.disabled ? currentInputElement.value : "";
                term.innerHTML = "";
                currentInputElement = null;
                currentInputId = null;
                currentErrorElement = null;
                currentInputContainer = null;
                let needMath = false;
                const frag = document.createDocumentFragment();
                const postTasks = [];
                for (let i = 0; i < data.output.length; i++) {
                    const line = String(data.output[i] ?? "");
                    if (line.startsWith("OBJECT_STAGE:")) {
                        const p = JSON.parse(line.slice(13));
                        let d = document.getElementById("ostage-" + p.id);
                        if (!d) {
                            d = document.createElement("div");
                            d.id = "ostage-" + p.id;
                            d.className = "object-stage";
                        }
                        frag.appendChild(d);
                        postTasks.push(() => webrustObjects.handleStage(p));
                        continue;
                    }
                    if (line.startsWith("OBJECT_CMDS:")) {
                        const p = JSON.parse(line.slice(12));
                        postTasks.push(() => webrustObjects.handleCmds(p));
                        continue;
                    }
                    if (line.startsWith("OBJECT_GROUP:")) {
                        const p = JSON.parse(line.slice(13));
                        postTasks.push(() => webrustObjects.handleGroup(p));
                        continue;
                    }
                    if (line.startsWith("SIMPLE_TABLE:")) {
                        const html = line.substring(13);
                        const div = document.createElement("div");
                        div.className = "table-container";
                        div.innerHTML = html;
                        frag.appendChild(div);
                        if (hasMath(html)) needMath = true;
                        continue;
                    }
                    if (line.startsWith("INPUT_REQUEST:")) {
                        const parts = line.split(":");
                        const id = parts[1];
                        const prompt = parts.slice(2).join(":");
                        const next = i + 1 < data.output.length ? String(data.output[i + 1] ?? "") : null;
                        const completed = next && !next.startsWith("INPUT_REQUEST:") && !next.startsWith("PROGRAM_FINISHED");
                        if (completed) {
                            const div = document.createElement("div");
                            div.className = "terminal-line";
                            div.innerHTML = `<span class="input-prompt">${prompt}</span> <span class="completed-input">${next}</span>`;
                            frag.appendChild(div);
                            if (hasMath(next)) needMath = true;
                            i++;
                            continue;
                        }
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
                            const v = inp.value.trim();
                            if (!v) { showError("Please enter a value"); inp.focus(); return; }
                            fetch("/api/validate", {
                                method: "POST",
                                headers: { "Content-Type": "application/json" },
                                body: JSON.stringify({ id: currentInputId, value: v }),
                            })
                                .then((r) => r.json())
                                .then((res) => {
                                    if (res?.valid) { clearError(); submitInput(); }
                                    else { showError(res?.error || "Invalid value"); inp.value = ""; inp.focus(); }
                                })
                                .catch(() => { showError("Validation failed"); inp.focus(); });
                        });
                        inp.addEventListener("input", () => {
                            if (inp.value.trim() && currentErrorElement) clearError();
                        });
                        row.appendChild(span);
                        row.appendChild(inp);
                        currentInputContainer.appendChild(row);
                        frag.appendChild(currentInputContainer);
                        postTasks.push(() => {
                            inp.focus();
                            const L = inp.value.length;
                            try { inp.setSelectionRange(L, L); } catch {}
                        });
                        continue;
                    }
                    const prev = i > 0 ? String(data.output[i - 1] ?? "") : null;
                    if (prev && prev.startsWith("INPUT_REQUEST:")) continue;
                    const div = document.createElement("div");
                    div.className = "terminal-line";
                    div.innerHTML = line;
                    frag.appendChild(div);
                    if (hasMath(line)) needMath = true;
                    if (line.includes("<script>")) {
                        postTasks.push(() => runInlineScripts(div));
                    }
                }
                term.appendChild(frag);
                postTasks.forEach((f) => f());
                if (needMath) renderMathJax(term);
                term.scrollTop = term.scrollHeight;
            })
            .catch((err) => {
                if (err?.name !== "AbortError") console.error("Error fetching state:", err);
            })
            .finally(() => { inFlight = null; });
    };
    const submitInput = () => {
        if (!currentInputId || !currentInputElement || processingInput) return;
        const v = currentInputElement.value;
        if (!v.trim()) return;
        processingInput = true;
        currentInputElement.disabled = true;
        fetch("/api/input", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ id: currentInputId, value: v }),
        })
            .then(() => {
                processingInput = false;
                currentInputId = null;
                currentInputElement = null;
                currentErrorElement = null;
                currentInputContainer = null;
                lastOutputLength = 0;
            })
            .catch(() => {
                processingInput = false;
                if (currentInputElement) currentInputElement.disabled = false;
            });
    };
    setInterval(updateDisplay, 300);
    updateDisplay();
})();