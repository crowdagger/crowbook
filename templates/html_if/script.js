var default_actions = {};
var state = {
    current_id: 0,
    visited: [],
    actions: {},
};

function perform_action(action) {
    var act = state.actions[action];
    if (act != null) {
	return act();
    } else {
	act = default_actions[action];
	if (act != null) {
	    return act();
	}
    }
}

function passageCount(n) {
    var count = state.visited.filter(v => v === n).length;
    console.log("passagecount(" + n + ") = " + count);
    return count;
}

{{common_script}}

var initFns = [];

{{new_game}}

{{js_prelude}}

function showChapter(chap, noreset){
    state.current_id = chap;
    console.log(state.visited);
    initFns[chap]();
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

window.onhashchange = function() {
    var hash = document.location.hash;
    if(!hash) {
        showChapter(0, true);
    } else {
        var element = document.getElementById(hash.substr(1));
	if(element) {
            var chap = getChapter(element);
            showChapter(chap, true);
	}
    }
};

window.onload = function(){
    displayAll = false;
    showChapter(0, true);
};
