// src/ThumbnailButton.tsx
import { useNavigate } from "react-router-dom";

const ThumbnailButton = () => {
  const navigate = useNavigate();
  const imageId = "https://upload.wikimedia.org/wikipedia/en/0/0b/Barbie_2023_poster.jpg";//"sample"; // You can use dynamic IDs or filenames
  const imagePathBase64 = btoa(imageId); // Base64 encode it

  return (
    <button
      className="p-2 border rounded hover:shadow"
      onClick={() => navigate(`/view/${imagePathBase64}`)}
    >
      <img
        src={`${imageId}`}
        alt="Thumbnail"
        className="w-32 h-32 object-cover"
      />
    </button>
  );
};

export default ThumbnailButton;
