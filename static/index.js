/* index.js
    author: Mary Montgomery
    date: 31 March 2020
*/

/*  loginSelection
    a function to press the button to log the user into
    their spotify
    @params
        button_id: the id of the button that was pressed
    @returns
        api_call: what ever the spotify api call is passed to 
                    somewhere else
*/
function loginSelection(button_id) {
    /*  get the button, and change the color to show selection */
    let selected_button = document.getElementById(button_id);
    selected_button.style.background = "black";
    selected_button.style.color = "#cfe8fa";

    /* on successful login, should changed to successfully logged in */
    /*might need to implement alerts for if the user is trying to press
        the music and time span buttons before logging in that just say like
        'you need to login before we can show you your data'            */

}

/*  musicSelection
    a function to find out what music button was pressed
    (and send that information to another function that makes
        the api call to spotify?)
    once one button is pressed the click features are disabled 
    from the other buttons, and the clicked button will change color.
    @params
        button_id: the id of the button that was pressed
    @returns
        user_music_selection: the needed spotify term for what music 
                            was selected
*/
function musicSelection(button_id) {
    /*  get the button, and change the color to show selection */
    let selected_button = document.getElementById(button_id);
    selected_button.style.background = "black";
    selected_button.style.color = "#cfe8fa";

    var artist_table = document.getElementById("artists");
    var track_table = document.getElementById("tracks");
    var chart_table = document.getElementById("charts");

    /* black out previously clicked buttons */
    if (button_id == "artist-music-button") {
        document.getElementById("song-music-button").style.background = "#cfe8fa";
        document.getElementById("song-music-button").style.color = "black";
        artist_table.style.display = "block";
        track_table.style.display = "none";
        chart_table.style.display = "none";
    }
    if (button_id == "song-music-button") {
        document.getElementById("artist-music-button").style.background = "#cfe8fa";
        document.getElementById("artist-music-button").style.color = "black";
        artist_table.style.display = "none";
        chart_table.style.display = "none";
        track_table.style.display = "block";
    }
    if (button_id == "chart-music-button") {
        document.getElementById("chart-music-button").style.background = "#cfe8fa";
        document.getElementById("chart-music-button").style.color = "black";
        artist_table.style.display = "none";
        track_table.style.display = "none";
        chart_table.style.display = "block";
    }

    /*  button_id looks like artist-music-button    */
    let selection_arr = button_id.split("-");
    console.log(selection_arr);
    /*  user_music_selection is just one word: artist, song, album, or genre */
    user_music_selection = selection_arr[0];
    console.log(user_music_selection);


    /* will need to return this information to send it somewhere */
}


/*  timeSelection
    a function to find out what time span button was pressed
    (and sends the information to another function that will make
        the spotify api call)
    once one button is clicked, the others will not have a click 
    feature and the clicked button will change color.
    @params
        button_id: the id of the clicked button
    @returns
        user_time_selection: the needed spotify term for the time
                            span the user selected
*/
function timeSelection(button_id) {
    /* get the button and change the color to show it is selected */
    let selected_button = document.getElementById(button_id);
    selected_button.style.background = "black";
    selected_button.style.color = "#cfe8fa";

    /* change the unselected buttons colors */
    if (button_id == "short-term-time-button" ||
        button_id == "medium-term-time-button") {
        document.getElementById("long-term-time-button").style.background = "#cfe8fa";
        document.getElementById("long-term-time-button").style.color = "black";
    }
    if (button_id == "short-term-time-button" ||
        button_id == "long-term-time-button") {
        document.getElementById("medium-term-time-button").style.background = "#cfe8fa";
        document.getElementById("medium-term-time-button").style.color = "black";
    }
    if (button_id == "medium-term-time-button" ||
        button_id == "long-term-time-button") {
        document.getElementById("short-term-time-button").style.background = "#cfe8fa";
        document.getElementById("short-term-time-button").style.color = "black";
    }

    /*  button_id looks like short-term-time-button    */
    let selection_arr = button_id.split("-");
    console.log(selection_arr);
    /*  user_time_selection is just one word: short, medium, or long */
    user_time_selection = selection_arr[0];
    console.log(user_time_selection);
}