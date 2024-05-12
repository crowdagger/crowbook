function on(name) {
    var elements = document.getElementsByClassName(name);
    for (var i = 0; i < elements.length; i++) {
        var elem = elements[i];
        elem.style.backgroundColor = "pink";
    }
}
function off(name) {
    var elements = document.getElementsByClassName(name);
    for (var i = 0; i < elements.length; i++) {
        var elem = elements[i];
        elem.style.backgroundColor = "white";
    }
}

var display_menu = false;
function toggle() {
    if (display_menu == true) {
        display_menu = false;
        document.getElementById("nav").style.left = "-21%";
        document.getElementById("content").style.marginLeft = "0%";
        document.getElementById("menu").style.left = "1em";
/*        if(document.getElementById("top")) {
            document.getElementById("top").style.left = "0";
        }
        if(document.getElementById("footer")) {
            document.getElementById("footer").style.marginLeft = "0%";
        }*/
    } else {
        display_menu = true;
        document.getElementById("nav").style.left = "0";
        document.getElementById("content").style.marginLeft = "20%";
        document.getElementById("menu").style.left = "20%";
/*        if(document.getElementById("top")) {
            document.getElementById("top").style.left = "20%";
        }
        if(document.getElementById("footer")) {
            document.getElementById("footer").style.marginLeft = "20%";
        }*/
    }
}

function remove_footnotes() {
    var footnotes = document.querySelectorAll('.popup_footnote');
    for (var i = 0; i < footnotes.length; i++) {
        var f = footnotes[i];
        if (f.parentNode) {
            f.parentNode.removeChild(f);
        }
    }
}

function show_footnote(event) {
    remove_footnotes();
    var id = event.target.getAttribute("id");
    var target_id = id.replace("source", "dest");
    var content = document.getElementById(target_id).innerHTML;
    var top = Math.round(event.target.getBoundingClientRect().top + event.target.getBoundingClientRect().height);
    document.getElementById(id).insertAdjacentHTML('afterend', '<aside class = "popup_footnote" style = "position: fixed; top: '+ top + 'px; ">' + content + '</aside>')
}


document.addEventListener('DOMContentLoaded', function() {
    var anchors = document.querySelectorAll('.footnote_reference');

    for (var i = 0; i < anchors.length; i++) {
        var anchor = anchors[i];
        anchor.addEventListener(
            "mouseenter",
            (event) => {
                show_footnote(event);
            }
        );
        anchor.addEventListener(
            "mouseleave",
            (event) => {
                remove_footnotes();
            }
        );
    }
});
