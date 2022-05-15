lsof -nP -iTCP -sTCP:LISTEN | grep -v COMMAND | tr -s ' ' | sort | cut -d ' ' -f 1,3,9 | awk ' \
    BEGIN { print "[" ; }  \
    { print " {\n" \
    "   \"Command\" : \""   $1  "\",\n"  \
    "   \"User\" : \""  $2 "\",\n"  \
    "   \"Local Address:Port\" : \""  $3  "\" \n"  \
    " }," } \
    END { print "]" } ' | tr --delete '\n' | sed 's/},]/}]/g'