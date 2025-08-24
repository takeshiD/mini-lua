for f in bitfields/*; do
    bn=$(basename "$f")
    svg_name="img/${bn%.*}.svg"
    uvx --from wavedrom wavedrompy --input "${f}" --svg "$svg_name"
    echo "[INFO] create $svg_name"
done
