import sys
import time
import pygame
import io
import spotipy
import spotipy.util as util
import spotipy.oauth2 as auth
from urllib2 import urlopen

### Do not change any of this
scope = 'user-read-currently-playing'
username = 'kylesurowiec'
id = '21a92e6b6b8a4b85a95dcec0a4f7387a'
secret = 'b44f27e90ba1476196eb2db9c6987207'
uri = 'http://localhost:8888/callback'
### --------------------------

token = util.prompt_for_user_token(username, scope, id, secret, uri)
pygame.init()
#print token

### Opens a pygame window, fills background and draws album cover centered.
### Monitor width and height passed through on call.
def draw_image(width, height, image_url):
    black = (0, 0, 0)
    width = width
    height = height
    image_height = 640
    image_width = 640
    scale = 1.2
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
### returns JSON, search json for album art link. call draw_image() to display it.
### Will need to be threaded or re-timed to update after a song changes
if token:
    sp = spotipy.Spotify(auth=token)
    results = sp.currently_playing()
    image_src = results["item"]["album"]["images"][0]["url"]
    draw_image(1920, 1080, image_src)

    current = image_src
    while True:
        time.sleep(1)
        results = sp.currently_playing()
        image_src = results["item"]["album"]["images"][0]["url"]
        if current != image_src:
            current = image_src
            draw_image(1920, 1080, image_src)
        else:
            print "same song"
 
    print image_src
else:
    print "Can't get token for", username
