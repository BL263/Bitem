import React, { useState } from 'react';
import { createItem } from '../api';
import ItemListPopup from './ItemListPopup';

const CreateItem = () => {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [price, setPrice] = useState('');
  const [showPopup, setShowPopup] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    const item = {
      title,
      description,
      price: parseFloat(price),
      seller_id: '000000000000000000000000' // example ObjectId
    };

    try {
      await createItem(item);
      alert('Item added successfully!');
      setTitle('');
      setDescription('');
      setPrice('');
      setShowPopup(true);
    } catch (error) {
      alert('Error adding item');
    }
  };

  return (
    <div className="max-w-xl mx-auto mt-8 bg-white dark:bg-gray-800 p-6 rounded-lg shadow">
      <h2 className="text-2xl font-bold mb-4 text-gray-900 dark:text-white">Add New Item</h2>
      <form onSubmit={handleSubmit} className="space-y-4">
        <input
          type="text"
          value={title}
          onChange={e => setTitle(e.target.value)}
          placeholder="Title"
          required
          className="w-full px-4 py-2 border border-gray-300 dark:border-gray-700 rounded bg-white dark:bg-gray-900 text-gray-900 dark:text-white"
        />
        <input
          type="text"
          value={description}
          onChange={e => setDescription(e.target.value)}
          placeholder="Description"
          required
          className="w-full px-4 py-2 border border-gray-300 dark:border-gray-700 rounded bg-white dark:bg-gray-900 text-gray-900 dark:text-white"
        />
        <input
          type="number"
          value={price}
          onChange={e => setPrice(e.target.value)}
          placeholder="Price"
          required
          className="w-full px-4 py-2 border border-gray-300 dark:border-gray-700 rounded bg-white dark:bg-gray-900 text-gray-900 dark:text-white"
        />
        <button
          type="submit"
          className="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded"
        >
          Add Item
        </button>
      </form>

      {showPopup && <ItemListPopup onClose={() => setShowPopup(false)} />}
    </div>
  );
};

export default CreateItem;
