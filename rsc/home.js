document.addEventListener("DOMContentLoaded", ()=>{
    update_list();
})

// filter data regarding title
// and edit the title to add `<mark>` for matching substring
function apply_title_query(data){
    let query = document.querySelector("nav input#title").value;
    return data.filter(d => d.title.toLowerCase().includes(query.toLowerCase()))
        .map(d => {
            const dd = { ...d };
            dd.title = dd.title.replace(new RegExp(query, "i"), match => `<mark>${match}</mark>`);
            return dd;
        });
}

// filter data regarding tech
// and edit the tech to add `<mark>` for matching substring
function apply_tech_query(data){
    let query = document.querySelector("nav input#tech").value.trim();
    return data.filter(d => d.tech.toLowerCase().startsWith(query.toLowerCase()))
        .map(d => {
            const dd = { ...d };
            dd.tech = dd.tech.replace(new RegExp(query, "i"), match => `<mark>${match}</mark>`);
            return dd;
        });
}

function create_element(data){
    let div = document.createElement("div");
    div.innerHTML = `
<a href="pages/${data.id}">
  <p class="title">${data.title}</p>
  <p class="tech">${data.tech}</p>
</a> 
<img onclick="toggle_fav(this,${data.id}); update_list()" class="favorite ${data.isfav? 'isfav' : ''}" src="rsc/assets/favorite.svg">`;
    return div;
}

function update_list(){
    let data = indexation;

    let main = document.querySelector("main");
    main.innerHTML = "";

    data = apply_tech_query(data);
    if (data.length === 0){
        document.querySelector("#result-separator p").innerHTML = "No results matching 'Tech' filter";
        document.querySelector("input#tech").classList.add("error");
        return;
    }
    document.querySelector("input#tech").classList.remove("error");

    data = apply_title_query(data);
    if (data.length === 0){
        document.querySelector("#result-separator p").innerHTML = "No results matching 'Title' filter";
        document.querySelector("input#title").classList.add("error");
        return;
    }
    document.querySelector("input#title").classList.remove("error");


    document.querySelector("#result-separator p").innerHTML = `${data.length}`;

    let raw_favs = localStorage.getItem('favs');
    let favs = raw_favs ? raw_favs.split(',') : [];
    data.forEach(d=>{
        d.isfav = favs.includes(d.id.toString());
    });

    data.sort((a, b) => {
        if (a.isfav !== b.isfav){
            return a.isfav ? -1 : 1;
        }
        return (a.title + a.tech).localeCompare(b.title + b.tech);
    });

    data.forEach(d => {
        main.appendChild(create_element(d));
    });
}