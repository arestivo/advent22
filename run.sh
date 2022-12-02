folder='received'

while getopts 'd:p:s' opt; do
 case "${opt}" in
  d) day="$OPTARG";;
  p) part="$OPTARG";;
  s) folder='sample';;
 esac
done

cargo run --bin day$day$part < inputs/$folder/$day.in