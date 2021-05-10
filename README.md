# ubersicht-unread-gmail
Show number of unread gmail using the gmail api and written in rust.

## Installation
Download and unzip the zip file.
Move it into the widgets directory for uebersicht to read.

## Multiple accounts
If you have multiple accounts you can make a copy of the widget and edit the first line of the index.coffee file.
The first link should take this format:
```
command: "relative_from_widgets_folder_path/to/mail_unread_counter -t='relative_from_widgets_folder_path/tokencache.json' -c='relative_from_widgets_folder_path/to/credentials.json' ..."
```
