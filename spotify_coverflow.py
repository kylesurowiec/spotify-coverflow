import json
import time
from io import BytesIO
import itunes
import spotipy
import spotipy.util as util
from Tkinter import *
from PIL import Image, ImageTk
from urllib2 import urlopen

with open('keys.json') as keys:
    data = json.load(keys)

# Spotify API credentials, Globals
scope = 'user-read-currently-playing'
username = data["username"]
id = data["id"]
secret = data["secret"]
uri = data["uri"]
width_height = [1920, 1080]
images = []
# --------------------------

# Send user credentials to Spotify Auth
def authorize():
    token = util.prompt_for_user_token(username, scope, id, secret, uri)
    return token

# Return information about the current playing song
def get_current_playing(token):
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

# Convert the image url to something that tkinter can use
def convert_image(src):
    image_str = urlopen(src).read()
    im = Image.open(BytesIO(image_str))
    image = ImageTk.PhotoImage(im)
    images.append(image)
    return image

def check_itunes_artwork(album, artist):
    i_artist = itunes.search_album(album, artist)[0]
    hd_img = i_artist.get_artwork()['100']
    hd_img = hd_img.replace('100x100bb', '1000x1000')
    return hd_img

def get_img_src():
    album = get_current_playing(authorize())[1]
    artist = get_current_playing(authorize())[2]
    try:
        src = check_itunes_artwork(album, artist)
        return src
    except:
        src = get_current_playing(authorize())[0]
        return src

# Draw the image into the tkinter window
def draw_window():
    root = Tk()
    root.configure(bg="black", cursor="none")
    root.attributes('-fullscreen', True)
    f = Frame(root, bg="black", width=1920, height=1080)
    f.grid(row=0, column=0, sticky="NW")
    f.grid_propagate(0)
    f.update()

    while True:
        img_url = get_img_src()
        image = convert_image(img_url)
        label = Label(f, image=image, highlightthickness=0, bd=0)
        label.place(x=550, y=540, anchor="center")
        size = 50
        length = len(get_current_playing(authorize())[3])
        if length > 18:
            size = 30
        if length > 30:
            size = 20
        name_label = Label(f, text=get_current_playing(authorize())[3], bg="black", fg="white", font=("Courier New", size))
        name_label.place(x=1500, y=540, anchor="center")
        artist_label = Label(f, text=get_current_playing(authorize())[2], bg="black", fg="white", font=("Courier New", 20))
        artist_label.place(x=1500, y=480, anchor="center")

        root.update_idletasks()
        root.update()
        time.sleep(4)
        label.destroy()
        name_label.destroy()
        artist_label.destroy()

if __name__ == "__main__":
    draw_window()
