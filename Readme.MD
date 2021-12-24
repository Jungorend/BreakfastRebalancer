# Captain Crunch Deck Updater

TTS with the Exceed Balanced Breakfast Module is an unofficial module for playing
the Exceed Engine.

When testing custom decks, it frequently becomes necessary to update the card sets that you're using.
Due to the requirements names have in order to use the reference generator, this can mean individual updates
can take a good extra couple minutes renaming each file to what it was before the change.

This tool is designed to make that process quicker.

Simply run:
`deck-updater.exe original-json new-json -a URL`

where URL is the new address of the attack cards you wish to use, and it will replace the cards with these
images without modifying the names. You can then move the new json to your TTS objects area and it should be available.