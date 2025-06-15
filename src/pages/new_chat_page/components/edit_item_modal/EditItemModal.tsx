import React from "react";
import "./EditItemModal.css";

type EditItemModalProps = {
  imageUrl: string;
  onClose: () => void;
};

const EditItemModal: React.FC<EditItemModalProps> = ({ imageUrl, onClose }) => {
  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <img src={imageUrl} alt="modal-img" className="modal-img" />
        <button className="close-button" onClick={onClose}>×</button>
      </div>
    </div>
  );
};

export default EditItemModal;
