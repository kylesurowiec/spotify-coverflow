#!python2

import json
import time
from io import BytesIO
import itunes
import spotipy
import spotipy.util as util
from Tkinter import *
from PIL import Image, ImageTk
from urllib2 import urlopen

# Load in keys for Spotify
with open('keys.json') as keys:
    data = json.load(keys)


# Spotify API credentials, Globals
scope = 'user-read-currently-playing'
username = data["username"]
id = data["id"]
secret = data["secret"]
uri = data["uri"]
width_height = [1920, 1080]
# --------------------------


def authorize():
    '''Send user credentials to Spotify Auth'''
    token = util.prompt_for_user_token(username, scope, id, secret, uri)
    return token


def get_current_playing(token):
    '''Return information about the current playing song'''
    if token:
        sp = spotipy.Spotify(auth=token)
        results = sp.currently_playing()
        image_src = results["item"]["album"]["images"][0]["url"]
        artist = results["item"]["album"]["artists"][0]["name"]
        album = results["item"]["album"]["name"]
        song = results["item"]["name"]
        return [image_src, album, artist, song]
    else:
        print "Can't get token!"


def convert_image(src):
    '''Convert the image url to Byte Array'''
    image_str = urlopen(src).read()
    im = Image.open(BytesIO(image_str))
    image = ImageTk.PhotoImage(im)
    return image


def check_itunes_artwork(album, artist):
    '''Search the iTunes API and grab the most popular result'''
    i_artist = itunes.search_album(album, artist)[0]
    hd_img = i_artist.get_artwork()['100']
    hd_img = hd_img.replace('100x100bb', '1000x1000')
    return hd_img


def get_img_src():
    '''
    Get the current playing song artwork url from Spotify,
    search iTunes for same album and artist to get artwork,
    return Spotify artwork if something goes wrong
    '''
    album = get_current_playing(authorize())[1]
    artist = get_current_playing(authorize())[2]
    try:
        src = check_itunes_artwork(album, artist)
        return src
    except Exception: # Something went wrong with iTunes, so switch the Spotify coverart
        src = get_current_playing(authorize())[0]
        return src


def draw_window():
    '''Main event loop, draw the image and text to tkinter window'''
    root = Tk()
    root.configure(bg="black", cursor="none")
    root.attributes('-fullscreen', True)
    f = Frame(root, bg="black", width=1920, height=1080)
    f.grid(row=0, column=0, sticky="NW")
    f.grid_propagate(0)
    f.update()

    # Main loop for drawing to window
    while True:
        img_url = get_img_src()
        # Convert img from url and place it into window
        image = convert_image(img_url)
        label = Label(f, image=image, highlightthickness=0, bd=0)
        label.place(x=550, y=540, anchor="center")
        # Size of song text label
        size = 50
        length = len(get_current_playing(authorize())[3])
        if length > 18:
            size = 30
        if length > 30:
            size = 20
        # Draw the labels
        name_label = Label(f, text=get_current_playing(authorize())[3], bg="black", fg="white", font=("Courier New", size))
        name_label.place(x=1500, y=540, anchor="center")
        artist_label = Label(f, text=get_current_playing(authorize())[2], bg="black", fg="white", font=("Courier New", 20))
        artist_label.place(x=1500, y=480, anchor="center")
        root.update_idletasks()
        root.update()
        time.sleep(4) # after 4 seconds destroy the previous image and labels so we can draw a new one
        label.destroy()
        name_label.destroy()
        artist_label.destroy()


if __name__ == "__main__":
    draw_window()
