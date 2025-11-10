
function hasTouch() {
    return 'ontouchstart' in document.documentElement  || navigator.maxTouchPoints > 0 || navigator.msMaxTouchPoints > 0;
}


// document.addEventListener("DOMContentLoaded", ()=>{
//   theme = localStorage.getItem("ui-theme");
//   set_theme_ui(theme? theme : 'dark');
// })

const fav_localstorage_key = 'favs';
function toggle_fav(element,id){
    
    const raw_favs = localStorage.getItem(fav_localstorage_key);
    let favorites = raw_favs ? raw_favs.split(',') : [];
    
    if (favorites.includes(id.toString())) {
        console.log(`removing ${id} from fav`);
        favorites = favorites.filter(id_ => id_ !== id.toString());
        element.classList.remove('isfav');
    } else {
        console.log(`adding ${id} from fav`);
        favorites.push(id.toString());
        element.classList.add('isfav');
    }
    localStorage.setItem("favs",favorites.join(','));
    console.log("favorites:", localStorage.getItem(fav_localstorage_key))
}


function set_theme_ui(theme) {

    console.log("switch to", theme);
    
    // button
    document.querySelectorAll(`[data-theme]`).forEach(d=>{d.classList.add("hide");});
    document.querySelector(`[data-theme="${theme === 'dark'? 'light' : 'dark'}"]`).classList.remove("hide");
    
    // stylesheet
    document.querySelectorAll("#style-ui-dark, #style-ui-light").forEach(s=>{s.disabled = true;});
    document.getElementById(`style-ui-${theme}`).disabled = false;
    
    localStorage.setItem('ui-theme', theme);
}




function hasTouch() {
    return 'ontouchstart' in document.documentElement  || navigator.maxTouchPoints > 0 || navigator.msMaxTouchPoints > 0;
}




