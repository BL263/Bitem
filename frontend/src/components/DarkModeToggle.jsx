import React, { useEffect, useState } from 'react';
import Cookies from 'js-cookie';

const DarkModeToggle = () => {
  const [enabled, setEnabled] = useState(false);
  const [consent, setConsent] = useState(null);

  useEffect(() => {
    const cookieConsent = Cookies.get('cookie-consent');
    if (cookieConsent === 'true') {
      setConsent(true);
    } else if (cookieConsent === 'false') {
      setConsent(false);
    }

    const savedTheme = Cookies.get('theme');
    if (savedTheme === 'dark') {
      document.documentElement.classList.add('dark');
      setEnabled(true);
    }
  }, []);

  const toggleDarkMode = () => {
    const html = document.documentElement;
    if (enabled) {
      html.classList.remove('dark');
      if (consent) Cookies.set('theme', 'light');
    } else {
      html.classList.add('dark');
      if (consent) Cookies.set('theme', 'dark');
    }
    setEnabled(!enabled);
  };

  const handleConsent = (allow) => {
    Cookies.set('cookie-consent', allow.toString(), { expires: 365 });
    setConsent(allow);
  };

  return (
    <>
      {/* Dark Mode Toggle Button */}
      <button
        onClick={toggleDarkMode}
        className="px-4 py-2 bg-gray-300 dark:bg-gray-800 dark:text-white rounded"
      >
        {enabled ? '☀️ Light Mode' : '🌙 Dark Mode'}
      </button>

      {/* Cookie Consent Banner */}
      {consent === null && (
        <div className="fixed bottom-0 left-0 right-0 bg-gray-200 dark:bg-gray-800 text-gray-900 dark:text-white px-4 py-3 shadow-md flex flex-col sm:flex-row justify-between items-center z-50">
          <p className="mb-2 sm:mb-0">We use cookies to remember your theme preference. Do you allow this?</p>
          <div className="flex gap-2">
            <button
              onClick={() => handleConsent(true)}
              className="bg-blue-600 text-white px-3 py-1 rounded hover:bg-blue-700"
            >
              Allow
            </button>
            <button
              onClick={() => handleConsent(false)}
              className="bg-gray-500 text-white px-3 py-1 rounded hover:bg-gray-600"
            >
              Deny
            </button>
          </div>
        </div>
      )}
    </>
  );
};

export default DarkModeToggle;
