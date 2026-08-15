function find_kindlegen() {
    bins=(
        ~/.wine/drive_c/users/pejuko/Local\ Settings/Application\ Data/Amazon/Kindle\ Previewer\ 3/lib/fc/bin/kindlegen.exe
        ~/.wine/drive_c/users/pejuko/AppData/Local/Amazon/Kindle\ Previewer\ 3/lib/fc/bin/kindlegen.exe
    );
    path=""
    
    for bin in "${bins[@]}" ; do
        if [ -e "$bin" ]; then
            path="$bin"
        fi
    done

    echo "$path"
}

KINDLEGEN=$(find_kindlegen)
if [ -z "$KINDLEGEN" ]; then
    echo "Can not find kindle gen."
    exit 1
fi
