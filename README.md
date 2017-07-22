# SpotifyCoverFlow

`Python 2.7`

This project requires a `keys.json` file, formatted like such: 

```
{
	"username": "xxx",
	"id": "xxx",
	"secret": "xxx",
	"uri": "http://localhost:8888/callback"
	
}
```

This project works with your personal Spotify devloper account, please register at [Spotify Developer](https://developer.spotify.com/my-applications/#!/).

-----

SpotifyCoverFlow is a simple script to display a full-screen high resolution image of your current playing (or most recent) song on 
Spotify. The intended use is to dedicate a Pi-Zero and monitor/screen to be an always on digital poster for your favorite music artwork.

![Example](http://i.imgur.com/FRqCD8r.png)

As it stands, Spotify only supports image artwork up to `640x640`. To counteract this, the current playing song is then searched through iTunes to grab artwork up to `10000x10000`. However, if the artwork can't be found on iTunes, the lower resolution Spotify artwork will be displayed.
