# Scryfall Archiver

A docker container for routinely archiving Scryfall's Magic: The Gathering database.


## Configuration

### Environment Variables

| Variable                        | Required | Description |
| :------------------------------ | :------: | :---------- |
|  `SA_DATA_DIR`                  |   Yes    | The directory where data is saved. Changing this means you **need** to change where your `Data` volume is mounted to |
|  `SA_ARCHIVE_SCHEDULE`          |   Yes    | A CRON schedule for archiving data. By default it is set to once a month. Anything more than weekly is probably excessive |
|  `SA_BACKUP_SMALL_IMAGE`        |   Yes    | Whether to download the `small` image for each card |
|  `SA_BACKUP_NORMAL_IMAGE`       |   Yes    | Whether to download the `normal` image for each card |
|  `SA_BACKUP_LARGE_IMAGE`        |   Yes    | Whether to download the `large` image for each card |
|  `SA_BACKUP_PNG_IMAGE`          |   Yes    | Whether to download the `png` image for each card |
|  `SA_BACKUP_ART_CROP_IMAGE`     |   Yes    | Whether to download the `art_crop` image for each card |
|  `SA_BACKUP_BORDER_CROP_IMAGE`  |   Yes    | Whether to download the `border_crop` image for each card |
|  `SA_VERSION`                   |   No     | The Archiver Version |
|  `RUST_LOG`                     |   No     | The log level |

### Volumes

|  Volume  | Mount Point | Description            |
|:-------: | :---------: | :--------------------- |
|   Data   |   `/data`   | This is where all of the archived data will be stored. 


## License
 - This program is licensed under the [GNU General Public License Version 3](https://www.gnu.org/licenses/#GPL)
 - Please provide appropriate credit for code usage

Copyright Travis Lane