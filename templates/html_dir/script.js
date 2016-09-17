var display_menu = false;
function toggle() {
    if (display_menu == true) {
        display_menu = false;
        document.getElementById("nav").style.left = "-20%";
        document.getElementById("content").style.marginLeft = "10%";
        document.getElementById("menu").style.left = "1em";
        document.getElementById("top").style.left = "0";
    } else {
        display_menu = true;
        document.getElementById("nav").style.left = "0";
        document.getElementById("content").style.marginLeft = "30%";
        document.getElementById("menu").style.left = "20%";
        document.getElementById("top").style.left = "20%";
    }
}
