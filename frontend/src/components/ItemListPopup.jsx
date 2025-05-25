import React, { useEffect, useState } from 'react';
import { getItems } from '../api';

const ItemListPopup = ({ onClose }) => {
  const [items, setItems] = useState([]);

  useEffect(() => {
    const loadItems = async () => {
      try {
        const res = await getItems();
        if (res && Array.isArray(res.data)) {
          setItems(res.data);
        } else {
          console.error('Unexpected response format:', res);
        }
      } catch (error) {
        console.error('Failed to load items:', error);
      }
    };
    loadItems();
  }, []);

  return (
    <div style={popupStyle}>
      <div style={modalStyle}>
        <h3>Items List</h3>
        <ul>
          {items.map(item => (
            <li key={item._id}>
              <strong>{item.title}</strong> - ${item.price}<br />
              <em>{item.description}</em>
            </li>
          ))}
        </ul>
        <button onClick={onClose}>Close</button>
      </div>
    </div>
  );
};

const popupStyle = {
  position: 'fixed', top: 0, left: 0,
  width: '100%', height: '100%',
  backgroundColor: 'rgba(0,0,0,0.5)',
  display: 'flex', alignItems: 'center', justifyContent: 'center',
};

const modalStyle = {
  backgroundColor: 'white',
  padding: '20px',
  borderRadius: '10px',
  width: '300px',
  maxHeight: '400px',
  overflowY: 'auto',
};

export default ItemListPopup;
