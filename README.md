# 🔭 Bottom-Line Up Front

This is meant to generate the contents to be used in [my other importation tooling](https://github.com/EspressoCake/ADIDNS_Parser) with `A` records haivng IP address values provided by the output of [ADIDNS dump](https://github.com/dirkjanm/adidnsdump).

## 🚀 Usage
```
adidns_walker --help
Usage: adidns_walker --dumpzone-folder-path <DUMPZONE_FOLDER_PATH> --output-parent-filepath <OUTPUT_PARENT_FILEPATH> --domain <DOMAIN_FQDN>

Options:
  -p, --dumpzone-folder-path <DUMPZONE_FOLDER_PATH>      The path of the dumpzone folder
  -o, --output-parent-filepath <OUTPUT_PARENT_FILEPATH>  The path to the desired output location.
  -d, --domain <DOMAIN>                                  The assumed FQDN of the domain.
  -h, --help                                             Print help
  -V, --version                                          Print version
```

## 🌱 Outputs

A text file with proper formatting via the `-o` argument.