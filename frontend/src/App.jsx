import React, { Routes, Route, Link } from 'react-router-dom';
import ItemList from './components/ItemList';
import CreateItem from './components/CreateItem';
import ItemDetails from './components/ItemDetails'; 
import './index.css';
import DarkModeToggle from './components/DarkModeToggle';

function App() {
  return (
    <div className="min-h-screen bg-white text-black dark:bg-gray-900 dark:text-white transition-colors duration-300">
    <header className="p-4 flex justify-between items-center border-b border-gray-200 dark:border-gray-700">
      <h1 className="text-3xl font-bold">Bitem</h1>
      <DarkModeToggle />
    </header>

    <nav className="p-4 border-b border-gray-200 dark:border-gray-700">
      <Link to="/" className="mr-4 text-blue-600 dark:text-blue-400 hover:underline">Home</Link>
      <Link to="/create" className="text-blue-600 dark:text-blue-400 hover:underline">Add Item</Link>
    </nav>

    <main className="p-4">
      <Routes>
        <Route path="/" element={<ItemList />} />
        <Route path="/create" element={<CreateItem />} />
        <Route path="/items/:id" element={<ItemDetails />} />
      </Routes>
    </main>
  </div>
  );
}

export default App;
