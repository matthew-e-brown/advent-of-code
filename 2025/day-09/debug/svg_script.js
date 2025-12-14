/// ------------------------------------------------------------------------------------------------
/// This JS script was what I used to *originally* find my answer to part 2.
///
/// It was written exactly like this (into a comment in `main.rs` at first) so that it could be
/// dumped into the DevTools console to generate rectangles on the currently previewed SVG. I never
/// meant for it to get this large; its original purpose was just to help me find edge-cases for
/// line intersections. But when I couldn't see through all the rectangles, I needed to start
/// culling boxes.
///
/// I used some manual placement (`ox,oy` is the center, for example; 269731960 was a previous
/// answer known to be too small) to remove some obvious rectangles, then a click event listener to
/// delete more and more rectangles until finally only a few were left. From there, it only took one
/// or two tries to get it right.
///
/// Even though I've since fixed the intersection problem, it only felt right to keep this code
/// here.
/// ------------------------------------------------------------------------------------------------
const rootSvg = document.querySelector('svg');
document.querySelector('g#debug')?.remove();
document.querySelector('g#known')?.remove();
const points = Array.prototype.map.call(document.querySelectorAll('g rect'), rect => {
    const x = (+rect.x.baseVal.valueAsString) + 0.5;
    const y = (+rect.y.baseVal.valueAsString) + 0.5;
    return [x,y];
});
function rectFromCorners([x1, y1], [x2, y2]) {
    const [xMin, xMax] = x1 <= x2 ? [x1, x2] : [x2, x1];
    const [yMin, yMax] = y1 <= y2 ? [y1, y2] : [y2, y1];
    const w = xMax - xMin;
    const h = yMax - yMin;
    return { x: xMin, y: yMin, w, h };
}
function rectFromRect(rect) {
    return {
        x: +rect.x.baseVal.valueAsString,
        y: +rect.y.baseVal.valueAsString,
        w: +rect.width.baseVal.valueAsString,
        h: +rect.height.baseVal.valueAsString,
    };
}
function relMousePos(event) {
    const { clientX: x, clientY: y } = event;
    const pt = new DOMPoint(x, y);
    const rel = pt.matrixTransform(rootSvg.getScreenCTM().inverse());
    console.log('Mouse click at pos:', [x, y], 'Converted to:', [rel.x, rel.y]);
    return [rel.x, rel.y];
}
function makeRectElem(rect) {
    const elem = document.createElementNS("http://www.w3.org/2000/svg", 'rect');
    elem.setAttribute('x', rect.x.toString());
    elem.setAttribute('y', rect.y.toString());
    elem.setAttribute('width', rect.w.toString());
    elem.setAttribute('height', rect.h.toString());
    elem.setAttribute('fill', 'red');
    elem.setAttribute('opacity', '0.1');
    elem.setAttribute('data-area', ((rect.w + 1) * (rect.h + 1)).toString());
    return elem;
}
function makeDotElem([x, y], fill = 'black', r = 100) {
    const elem = document.createElementNS("http://www.w3.org/2000/svg", 'circle');
    elem.setAttribute('cx', x.toString());
    elem.setAttribute('cy', y.toString());
    elem.setAttribute('r', r.toString());
    elem.setAttribute('fill', fill);
    return elem;
}
function rectContains(rect, [x, y]) {
    const { x: rx, y: ry, w, h } = rect;
    return (rx <= x && x <= rx + w && ry <= y && y <= ry + h);
}
function rectCorners(rect) {
    const { x, y, w, h } = rect;
    return [[x, y], [x + w, y], [x + w, y + h], [x, y + h]];
}
const ox = (94926+1746)/2;
const oy = (50372+48373)/2;
function distFromCenter([x, y]) {
    const dx = ox - x;
    const dy = oy - y;
    return Math.sqrt(dx * dx + dy * dy);
}
const knownBad = []; // Array of points inside the big hole
for (let x = 1746; x <= 94926 - 10; x += 250) {
    // From left to right, add a point at the top and the bottom
    knownBad.push([x, 50372 - 10]);
    knownBad.push([x, 48373 + 10]);
}
knownBad.push([ox, oy]);
const newGroup = document.createElementNS("http://www.w3.org/2000/svg", 'g');
const badGroup = document.createElementNS("http://www.w3.org/2000/svg", 'g');
newGroup.setAttribute('id', 'debug');
badGroup.setAttribute('id', 'known');
let n = 0;
for (let i = 0; i < points.length; i++) {
    for (let j = i + 1; j < points.length; j++) {
        const rect = rectFromCorners(points[i], points[j]);
        const area = ((rect.w + 1) * (rect.h + 1));
        if (
            true
            && area >= 269731960
            && !knownBad.some(pt => rectContains(rect, pt))
            && rectCorners(rect).every(pt => distFromCenter(pt) <= 50000)
        ) {
            const elem = makeRectElem(rect);
            elem.setAttribute('id', `rect-${n++}`);
            elem.setAttribute('data-i', i.toString());
            elem.setAttribute('data-j', j.toString());
            newGroup.append(elem);
        }
    }
}
for (const bad of knownBad) {
    badGroup.append(makeDotElem(bad));
}
rootSvg.addEventListener('click', event => {
    console.log('Click detected...');
    const mousePos = relMousePos(event);
    let n = 0;
    let child = newGroup.children[0];
    while (child) {
        const next = child.nextElementSibling;
        if (rectContains(rectFromRect(child), mousePos)) {
            child.remove();
            n++;
        }
        child = next;
    }
    badGroup.append(makeDotElem(mousePos, 'blue'));
    console.log('Click processed, removed', n, 'children');
});
rootSvg.append(newGroup);
rootSvg.append(badGroup);
function getSortedAreas() {
    return [...newGroup.children]
        .map(rect => ({ rect, area: +rect.getAttribute('data-area') }))
        .toSorted((a, b) => b.area - a.area);
}
