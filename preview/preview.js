const customRenderer = {
    code(token) {
        const code = token.text;
        const lang = token.lang;

        return `
<div class="code-block">
  <div class="code-header">
    <p class="language">${validLang}</p>
    <div><img class="copy" src="../rsc/assets/copy.svg"></div>
  </div>
  <pre><code class="language-${validLang}">${highlightedCode}</code></pre>
</div>`;
    }
};


function render() {
    const markdownInput = document.getElementById('markdown-input').value;
    const htmlOutputDiv = document.getElementById('render-html');
    
    const markedInstance = new marked.Marked();
    markedInstance.use({ 
        renderer: customRenderer, 
        gfm: true
    });
    const htmlResult = markedInstance.parse(markdownInput);
    htmlOutputDiv.innerHTML = htmlResult;
}


function show(id){
  document.getElementById('markdown').classList.add("hide");
  document.getElementById('render').classList.add("hide");
  document.getElementById(id).classList.remove("hide");
  
  document.querySelectorAll('#selector button').forEach(b => b.classList.remove("actif"));
  document.querySelector(`#selector button.${id}`).classList.add("actif");
}




document.addEventListener('DOMContentLoaded', function() {
  show('markdown')
});
