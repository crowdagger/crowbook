/* Builtin script also includes html_dir.script */
{{#display_chapter}}
function showChapter(chap,noreset){
    if (!displayAll) {
        var chapters = document.getElementsByClassName("chapter");
        for (i = 0; i < chapters.length; i++) {
            if (i == chap){
                chapters[i].style.display = "block";
            }else{
                chapters[i].style.display = "none";
            }
        }
        var controls = document.getElementsByClassName("chapterControls");
        for (i = 0; i < controls.length; i++){
            if (i>=chap*2-1 && i<=chap*2){
                controls[i].style.display = "block";
            }else{
                controls[i].style.display = "none";
            }
        }
        if (!noreset) {
            window.location.hash = "#chapter-"+chap;
        }
    }else{
        window.location.hash = "#chapter-"+chap;
    }
}

function switchAll(){
    if (!displayAll){
        displayAll = true;
        var chapters = document.getElementsByClassName("chapter");
        for (i = 0; i < chapters.length; i++) {
            chapters[i].style.display = "block";
        }
        var controls = document.getElementsByClassName("chapterControls");
        for (i = 0; i < controls.length; i++){
            controls[i].style.display = "none";
        }
        displayAllSwitcher = document.getElementById("book-button");
        displayAllSwitcher.src="{{{pages_svg}}}";
    }else{
        displayAll = false;
        showChapter(0);
        displayAllSwitcher = document.getElementById("book-button");
        displayAllSwitcher.src="{{{book_svg}}}";
    }
}

window.onload = function(){
    displayAll = false;
    showChapter(0,true);
};
{{/display_chapter}}