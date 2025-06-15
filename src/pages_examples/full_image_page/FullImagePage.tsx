// src/FullImagePage.tsx
import { useParams, useNavigate } from "react-router-dom";

const FullImagePage = () => {
  const { imageId } = useParams();
  const navigate = useNavigate();

  if (!imageId) return <div>Error: No image ID provided</div>;

  let decodedPath = "";
  try {
    decodedPath = atob(imageId); // decode Base64
  } catch (e) {
    return <div>Invalid image path.</div>;
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-black text-white">
      <button onClick={() => navigate(-1)} className="mb-4 text-blue-400 underline">
        Go Back
      </button>
      <img
        src={decodedPath}
        alt="Full Image"
        className="max-w-full max-h-screen object-contain"
      />
    </div>
  );
};

export default FullImagePage;
