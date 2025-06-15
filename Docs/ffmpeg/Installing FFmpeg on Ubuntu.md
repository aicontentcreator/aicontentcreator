# Installing FFmpeg on Ubuntu

You can install FFmpeg on Ubuntu using the package manager. Here's how:

1. Open a terminal window
2. Update your package lists:
   ```
   sudo apt update
   ```

3. Install FFmpeg:
   ```
   sudo apt install ffmpeg
   ```

4. Verify the installation:
   ```
   ffmpeg -version
   ```

That's it! FFmpeg should now be installed and ready to use on your Ubuntu system.

If you need a newer version than what's available in the default repositories, you can use a PPA:

```
sudo add-apt-repository ppa:savoury1/ffmpeg4
sudo apt update
sudo apt install ffmpeg
```

Would you like me to explain any specific FFmpeg functionality or commands?
