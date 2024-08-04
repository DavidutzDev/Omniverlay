const tooltipElements = document.querySelectorAll('[data-bs-toggle="tooltip"]');
tooltipElements.forEach((t) => {
    var e = t.getAttribute("title"),
        o = t.getAttribute("data-bs-placement") || "top",
        a = t.getAttribute("data-bs-trigger") || "hover";
    const r = document.createElement("div");
    r.classList.add("tooltip"), r.setAttribute("role", "tooltip");
    const n = document.createElement("i");
    n.classList.add("fa", "fa-chevron-down"),
        n.setAttribute("data-popper-arrow", ""),
        r.appendChild(n);
    const s = document.createElement("div");
    s.classList.add("tooltip-inner"),
        (s.textContent = e),
        r.appendChild(s),
        document.body.appendChild(r);
    const d = Popper.createPopper(t, r, {
        placement: o,
        modifiers: [
            {
                name: "offset",
                options: { offset: [0, 10] },
                preventOverflow: { boundariesElement: "viewport" },
            },
        ],
    });
    "hover" === a
        ? (t.addEventListener("mouseenter", () => {
              console.log("enter"), r.setAttribute("data-show", ""), d.update();
          }),
          t.addEventListener("mouseleave", () => {
              console.log("leave"), r.removeAttribute("data-show");
          }))
        : "click" === a &&
          t.addEventListener("click", () => {
              r.hasAttribute("data-show")
                  ? r.removeAttribute("data-show")
                  : (r.setAttribute("data-show", ""), d.update());
          });
});