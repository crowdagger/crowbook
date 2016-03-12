/* Builtin script also includes html_dir.script */
function showChapter(chap,noreset) {
    var chapters = document.getElementsByClassName("chapter");
    for (i = 0; i < chapters.length; i++) {
        if (i == chap) {
            chapters[i].style.display = "block";
        } else {
            chapters[i].style.display = "none";
        }
    }
    var controls = document.getElementsByClassName("chapterControls");
    for (i = 0; i < controls.length; i++) {
        if (i>=chap*2-1 && i<=chap*2){
            controls[i].style.display = "block";
        } else {
            controls[i].style.display = "none";
        }
    }
    if (!noreset) {
        window.location.hash = "#chapter-"+chap;
    }
}

window.onload = function(){
    showChapter(0, true);
    var controls = document.getElementsByClassName("chapterControls");
};
