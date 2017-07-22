import sys
import json
import select
import time
import pygame
import io
import itunes
import spotipy
import spotipy.util as util
import spotipy.oauth2 as auth
from urllib2 import urlopen
from msvcrt import getch

with open('keys.json') as keys:
    data = json.load(keys)

### Spotify API credentials
scope = 'user-read-currently-playing'
username = data["username"]
id = data["id"]
secret = data["secret"]
uri = data["uri"]
### --------------------------

token = util.prompt_for_user_token(username, scope, id, secret, uri)
pygame.init()

### Opens a pygame window, fills background and draws album cover centered.
### image width and height passed through on call.
def draw_image(i_width, i_height, image_url, scale_bool):
    black = (0, 0, 0)
    width = 1920
    height = 1080 
    image_height = i_height
    image_width = i_width

    if scale_bool:
        scale = 1.2
    else:
        scale = 1.0
        
    screen = pygame.display.set_mode((width, height), pygame.FULLSCREEN)
    screen.fill(black)
    pygame.mouse.set_visible(False)
    image_str = urlopen(image_url).read()
    image_file = io.BytesIO(image_str)
    image = pygame.image.load(image_file).convert()
    image_resize = pygame.transform.scale(image, (int(image_width * scale), int(image_width * scale)))
    screen.blit(image_resize, (width / 2 - (image_width * scale / 2), height / 2 - (image_height * scale / 2)))
    pygame.display.flip()

            
### if spotify oauth token and scope are valid, gets current playing song.
### returns JSON object, search json for album art link. call draw_image() to display it.
if token:
    sp = spotipy.Spotify(auth = token)  
    current = ""
    while True:
        time.sleep(4)
        results = sp.currently_playing()
        image_src = results["item"]["album"]["images"][0]["url"]
        artist = results["item"]["album"]["artists"][0]["name"]
        album = results["item"]["album"]["name"]

        if current != image_src: 
            try:
                i_artist = itunes.search_album(album, artist)[0];
                hd_src = i_artist.get_artwork()['100']
                hd_src = hd_src.replace('100x100bb', '100000x100000-99')
                draw_image(1000, 1000, hd_src, False)
            except:
                print "Unexpected error:", sys.exc_info()[0]
                draw_image(640, 640, image_src, True)
            current = image_src
        else:
            print "same song"
        
else:
    print "Can't get token for", username
