import React, { useParams ,Link} from 'react-router-dom';
import { useEffect, useState } from 'react';

import axios from 'axios';

const ItemDetails = () => {
  const { id } = useParams();
  const [item, setItem] = useState(null);

  useEffect(() => {
    axios.get(`http://localhost:8080/api/item/${id}`)
      .then(res => setItem(res.data))
      .catch(err => console.error('Error fetching item:', err));
  }, [id]);

  if (!item) return <div className="p-4">Loading...</div>;

  return (
    <div className="max-w-3xl mx-auto bg-white shadow-xl  p-6 mt-6 rounded-lg dark:bg-gray-900 dark:text-white dark:shadow-xxl-white dark:border dark:border-gray-700">
      <h1 className="text-2xl font-bold mb-2">{item.title}</h1>
      <p className="text-gray-600 mb-4">{item.description}</p>
      <p className="text-blue-600 font-semibold text-lg mb-4">${item.price.toFixed(2)}</p>
      <p className="text-sm text-gray-400">Product ID: {item._id?.$oid}</p>
      <Link to={`/`} className="block mt-2 bg-blue-500 text-white text-center py-1 rounded hover:bg-blue-600">
      Back To Home
    </Link>
    </div>
  );
};

export default ItemDetails;
