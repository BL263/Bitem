import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { getItems } from '../api';
import Payment from './Payment';
import Icon from "../styles/Icon";

const ItemList = () => {
  const [items, setItems] = useState([]);
  const [showPaymentFor, setShowPaymentFor] = useState(false);

  const handleModalPayment = () => {
    setShowPaymentFor(prev=> !prev);
  };

  useEffect(() => {
    const fetchItems = async () => {
      const res = await getItems();
      setItems(res.data);
    };
    fetchItems();
  }, []);

  return (
    <div className="min-h-screen bg-gray-100 flex rounded-xl">

      {/* Left: Filters */}
      <aside className="w-64 bg-white p-4 shadow-lg dark:bg-gray-800 dark:text-white rounded-l-xl">
        <h2 className="text-xl font-bold mb-4">Filters</h2>
        <div className="mb-2">
          <label className="block text-sm font-medium text-gray-700">Category</label>
          <select className="mt-1 w-full border border-gray-300 rounded px-2 py-1">
            <option>All</option>
            <option>Electronics</option>
            <option>Fashion</option>
          </select>
        </div>
        <div className="mb-2">
          <label className="block text-sm font-medium text-gray-700">Price Range</label>
          <input type="range" className="w-full" />
        </div>
      </aside>

      {/* Center: Products */}
      <main className="flex-1 px-8 py-6 dark:bg-gray-900 dark:text-white">
        <div className="max-w-6xl mx-auto">
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            {items.map((item) => (
              <div
                key={item._id?.$oid || item._id}
                className="bg-white p-4 rounded-xl shadow hover:shadow-lg transition dark:bg-gray-800 dark:text-white"
              >
                <h3 className="text-lg font-semibold">{item.title}</h3>
                <p className="text-gray-600">{item.description}</p>
                <span className="text-blue-600 font-bold">${item.price}</span>
                <Link
                  to={`/items/${item._id?.$oid || item._id}`}
                  className="block mt-2 bg-blue-500 text-white text-center py-1 rounded hover:bg-blue-600"
                >
                  View Details
                </Link>

                <div className="mt-2">
                  <button
                    className="flex items-center space-x-1 text-yellow-500 hover:text-yellow-600 focus:outline-none"
                    title="Add to favorites"
                  >
                    <div className="flex items-center justify-center w-6 h-6 rounded-full">
                      <Icon icon="star-empty" size={40} color="blue" />
                    </div>
                  </button>
                  <button
                    id='payment-modal-root'
                    className="block mt-2 bg-green-500 text-white text-center py-1 rounded hover:bg-green-600 w-full"
                    onClick={handleModalPayment}
                  >
                    Buy
                  </button>
                  {showPaymentFor && (
                  <Payment
                  key={item._id?.$oid || item._id}
                    selectedItem={item}
                    showCardPopup={showPaymentFor}
                    handleCardPopup={handleModalPayment}
                  />
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      </main>

      {/* Right: Empty (for symmetry) */}
      <aside className="w-64 rounded-xl bg-gray-200 dark:bg-gray-800 dark:text-white">
        <p className="text-gray-500 text-center mt-4">Empty Space</p>
      </aside>
    </div>
  );
};

export default ItemList;