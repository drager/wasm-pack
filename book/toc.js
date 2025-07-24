// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="introduction.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="quickstart.html"><strong aria-hidden="true">2.</strong> Quickstart</a></li><li class="chapter-item expanded "><a href="prerequisites/index.html"><strong aria-hidden="true">3.</strong> Prerequisites</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="prerequisites/npm.html"><strong aria-hidden="true">3.1.</strong> npm (optional)</a></li><li class="chapter-item expanded "><a href="prerequisites/considerations.html"><strong aria-hidden="true">3.2.</strong> considerations</a></li><li class="chapter-item expanded "><a href="prerequisites/non-rustup-setups.html"><strong aria-hidden="true">3.3.</strong> Non-rustup setups</a></li></ol></li><li class="chapter-item expanded "><a href="commands/index.html"><strong aria-hidden="true">4.</strong> Commands</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="commands/new.html"><strong aria-hidden="true">4.1.</strong> new</a></li><li class="chapter-item expanded "><a href="commands/build.html"><strong aria-hidden="true">4.2.</strong> build</a></li><li class="chapter-item expanded "><a href="commands/test.html"><strong aria-hidden="true">4.3.</strong> test</a></li><li class="chapter-item expanded "><a href="commands/pack-and-publish.html"><strong aria-hidden="true">4.4.</strong> pack and publish</a></li><li class="chapter-item expanded "><a href="commands/init.html"><strong aria-hidden="true">4.5.</strong> init (DEPRECATED)</a></li></ol></li><li class="chapter-item expanded "><a href="tutorials/index.html"><strong aria-hidden="true">5.</strong> Tutorials</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/hybrid-applications-with-webpack/index.html"><strong aria-hidden="true">5.1.</strong> Hybrid applications with Webpack</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/hybrid-applications-with-webpack/getting-started.html"><strong aria-hidden="true">5.1.1.</strong> Getting started</a></li><li class="chapter-item expanded "><a href="tutorials/hybrid-applications-with-webpack/using-your-library.html"><strong aria-hidden="true">5.1.2.</strong> Using your library</a></li></ol></li><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/index.html"><strong aria-hidden="true">5.2.</strong> npm browser packages</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/getting-started.html"><strong aria-hidden="true">5.2.1.</strong> Getting started</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/getting-started/manual-setup.html"><strong aria-hidden="true">5.2.1.1.</strong> Manual Setup</a></li></ol></li><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/template-deep-dive/index.html"><strong aria-hidden="true">5.2.2.</strong> Template deep dive</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/template-deep-dive/cargo-toml.html"><strong aria-hidden="true">5.2.2.1.</strong> Cargo.toml</a></li><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/template-deep-dive/src-lib-rs.html"><strong aria-hidden="true">5.2.2.2.</strong> src/lib.rs</a></li><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/template-deep-dive/src-utils-rs.html"><strong aria-hidden="true">5.2.2.3.</strong> src/utils.rs</a></li><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/template-deep-dive/wee_alloc.html"><strong aria-hidden="true">5.2.2.4.</strong> wee_alloc</a></li><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/template-deep-dive/tests-web-rs.html"><strong aria-hidden="true">5.2.2.5.</strong> tests/web.rs</a></li></ol></li><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/building-your-project.html"><strong aria-hidden="true">5.2.3.</strong> Building your project</a></li><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/testing-your-project.html"><strong aria-hidden="true">5.2.4.</strong> Testing your project</a></li><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/packaging-and-publishing.html"><strong aria-hidden="true">5.2.5.</strong> Packaging and publishing</a></li><li class="chapter-item expanded "><a href="tutorials/npm-browser-packages/using-your-library.html"><strong aria-hidden="true">5.2.6.</strong> Using your library</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="cargo-toml-configuration.html"><strong aria-hidden="true">6.</strong> Cargo.toml Configuration</a></li><li class="chapter-item expanded "><a href="contributing.html"><strong aria-hidden="true">7.</strong> Contributing</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
