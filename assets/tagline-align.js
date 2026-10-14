// The quotation line finishes flush against the right edge of whatever the
// statement line above it currently ends at. That edge moves whenever the
// statement wraps, so it is measured here and published as a custom property.
(function () {
    function align() {
        var block = document.querySelector(".tagline-block");
        var sub = document.querySelector(".t-sub");
        var t2 = document.querySelector(".t2");
        if (!block || !sub || !t2) return;
        var range = document.createRange();
        range.selectNode(t2);
        var rects = range.getClientRects();
        if (!rects.length) return;
        var statementRight = rects[rects.length - 1].right;
        var blockRight = block.getBoundingClientRect().right;
        var inset = Math.max(0, blockRight - statementRight);
        sub.style.setProperty("--tail-inset", inset + "px");
    }
    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", align);
    } else {
        align();
    }
    if (typeof ResizeObserver === "function") {
        new ResizeObserver(align).observe(document.documentElement);
    } else {
        window.addEventListener("resize", align);
    }
})();
