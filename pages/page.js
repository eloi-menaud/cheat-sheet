
function init_code_copy_event(){
  document.querySelectorAll("div.code-block div.code-header img.copy").forEach(img => {
    img.addEventListener("click", () => {

      const code = img.closest("div.code-block").querySelector("pre");
      if (!code) return;
      
      navigator.clipboard.writeText(code.textContent).then(() => {
        img.classList.remove("copied");
        void img.offsetWidth; // force reflow
        img.classList.add("copied");
      }).catch(err => {
        img.classList.remove("copied");
        void img.offsetWidth;
        console.error("Copy failed", err);
      });
    });
  });
}


document.addEventListener("DOMContentLoaded", () => {
  init_code_copy_event();
  
  const main = document.querySelector("main");

  if (main) {
    const snippet = main.querySelector("#snippet");
    const details = main.querySelector("#details");
    const typeSelector = document.getElementById("type-selector");

    if (!(snippet && details && typeSelector)) {
      typeSelector.style.display = "none";
    }
  }
  

});




function show(button) {
  const targetId = button.dataset.target;

  const snippet = document.getElementById("snippet");
  const details = document.getElementById("details");
  const target = document.getElementById(targetId);

  snippet?.classList.add("hide");
  details?.classList.add("hide");

  target?.classList.remove("hide");

  document.querySelectorAll("#type-selector button").forEach(button => button.classList.remove("actif"))
  button.classList.add("actif")
}
