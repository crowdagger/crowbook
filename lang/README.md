# Language directory

This directory, and its subdirectories, contains the files that allow Crowbook to be translated in other languages:

* the
  [document](document/)
  directory contains YAML files that are used to translate the (few) strings that Crowbook inserts in the generated documents according to the `lang` option, such as the translations for "Chapter", or  "Table of Contents".
  To add a new translation, just translate the
  [document/en.yaml](document/en.yaml)
  into a new language.
  This is probably the easier translation job to do, and also the most important, since these strings are embedded in the generated documents.
* the
  [lib](lib/)
  and
  [bin](bin/)
  directories respectively contains PO files that are used to translate the strings used in Crowbook's library and binary.
  These files contain a lot more strings, and are only used when Crowbook is run (that is, if you don't have a translation for your language here, you may have to use Crowbook in English, but as long as there is a translation in the
  [document](document/)
  directory it will at least display the correct translation in the documents it generates).
