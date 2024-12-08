import base64
import sys
import fitz
from PIL import Image

def parse_cover_image(pdf_base64_content, output_image_path):
    pdf_doc = fitz.open(pdf_base64_content)
    first_page = pdf_doc[0]
    image_list = first_page.get_images(full=True)

    if image_list:
        xref = image_list[0][0]
        image = pdf_doc.extract_image(xref)

        image_bytes = image["image"]

        with open(output_image_path, "wb") as f:
            f.write(image_bytes)
        print("Cover image saved as", output_image_path)
    else:
        # No cover image found, render the first page as an image
        pix = first_page.get_pixmap()
        img = Image.frombytes("RGB", [pix.width, pix.height], pix.samples)
        img.save(output_image_path)
        print("First page rendered and saved as", output_image_path)
# pdf parse service

def main():
    # argv[1]: uuid
    file_uuid = sys.argv[1]
    pdf_path = "./temp/files/" + file_uuid + ".pdf"
    output_image_path = "./temp/covers/" + file_uuid + ".jpg"
    print("Parsing cover image from", pdf_path)
    parse_cover_image(pdf_path, output_image_path)

if __name__ == "__main__":
    main()