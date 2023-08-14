{{common_script}}

{% if one_chapter %}
function showChapter(chap, noreset){
    if (!displayAll) {
        var chapters = document.getElementsByClassName("chapter");
        for (i = 0; i < chapters.length; i++) {
            if (i == chap) {
                chapters[i].style.display = "block";
            } else {
                chapters[i].style.display = "none";
            }
        }
        var controls = document.getElementsByClassName("chapterControls");
        for (i = 0; i < controls.length; i++){
            if (i >= chap * 2-1 && i <= chap * 2) {
                controls[i].style.display = "block";
            } else {
                controls[i].style.display = "none";
            }
        }
	// Hide toc unless we're at first chapter
	var toc = document.getElementById("toc");
	if (toc && chap == 0) {
	    toc.style.display = "block";
	}
	if (toc && chap != 0) {
	    toc.style.display = "none";
	}
        if (!noreset) {
            window.location.hash = "#chapter-"+chap;
        }
    } else {
        window.location.hash = "#chapter-"+chap;
    }
}

function getChapter(elem) {
    if(!elem) {
        return 0;
    }
    if(elem.className == "chapter") {
        return parseInt(elem.id.substr("chapter-".length));
    } else {
        return getChapter(elem.parentElement);
    }
}

function switchAll() {
    if (!displayAll) {
        displayAll = true;
        var chapters = document.getElementsByClassName("chapter");
        for (i = 0; i < chapters.length; i++) {
            chapters[i].style.display = "block";
        }
        var controls = document.getElementsByClassName("chapterControls");
        for (i = 0; i < controls.length; i++){
            controls[i].style.display = "none";
        }
	var toc = document.getElementById("toc");
	if (toc) {
	    toc.style.display = "block";
	}
        displayAllSwitcher = document.getElementById("book-button");
        displayAllSwitcher.src="{{pages_svg}}";
        displayAllSwitcher.alt="{{loc_display_one}}";
        displayAllSwitcher.title="{{loc_display_one}}";
    } else {
        displayAll = false;
        showChapter(0);
        displayAllSwitcher = document.getElementById("book-button");
        displayAllSwitcher.src="{{book_svg}}";
        displayAllSwitcher.alt="{{loc_display_all}}";
        displayAllSwitcher.title="{{loc_display_all}}";
    }
}

window.onhashchange = function() {
    var hash = document.location.hash;
    if(!hash) {
        showChapter(0, true);
    } else {
        var element = document.getElementById(hash.substr(1));
        var chap = getChapter(element);
        showChapter(chap, true);
    }
};

window.onload = function(){
    displayAll = false;
    var hash = document.location.hash;
    if(!hash) {
        showChapter(0, true);
    } else {
        var element = document.getElementById(hash.substr(1));
        var chap = getChapter(element);
        showChapter(chap, true);
    }

};

{% endif %}
