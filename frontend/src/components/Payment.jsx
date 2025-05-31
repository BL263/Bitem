import React from "react";
import ReactDOM from "react-dom";
import { Elements, CardElement, useStripe, useElements } from "@stripe/react-stripe-js";
import { loadStripe } from "@stripe/stripe-js";
import { buyItem } from "../api";

const stripePromise = loadStripe("pk_test_51RSfs9IMKScAQ1iqcE9KxcKmRXqUXMuIsNratkaEe1xnaNmA7nUYIFBuq4ExaNFBdgzamGSAvz0PKVcMCt0iUYpi00Tt3KU01C"); // your public key

const PaymentModal = ({ selectedItem, handleCardPopup }) => {
  const stripe = useStripe();
  const elements = useElements();

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!stripe || !elements) return;

    const card = elements.getElement(CardElement);
    const { error, paymentMethod } = await stripe.createPaymentMethod({
      type: "card",
      card,
    });

    console.log("Payment Method:", paymentMethod);
    console.log("Error:", error);
    if (error) {
      alert(error.message);
      return;
    }
    
    try {
      const validPid = /^[0-9a-fA-F]{24}$/.test(paymentMethod.id) ? paymentMethod.id : null;
      if (!validPid) {
        alert("Invalid payment method ID format.");
        return;
      }

      await buyItem({
        iid: selectedItem._id, // Ensure field names match API expectations
        pid: validPid, // Correct field name for `pid`
      });
      alert("Payment successful!");
      handleCardPopup(false);
    } catch (err) {
      alert("Payment failed: " + (err.response?.data || err.message));
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-gray-800 p-6 rounded-xl w-full max-w-md relative shadow-lg">
        <button
          onClick={() => handleCardPopup(false)}
          className="absolute top-2 right-2 text-gray-500 hover:text-red-500"
        >
          ✕
        </button>
        <h2 className="text-xl font-bold mb-4">Pay for: {selectedItem.title}</h2>
        <p className="mb-4">Price: ${selectedItem.price}</p>

        <form onSubmit={handleSubmit} className="space-y-4">
          <CardElement
            className="p-3 border rounded bg-white"
            options={{
              style: {
                base: {
                  fontSize: "16px",
                  color: "#32325d",
                  "::placeholder": { color: "#a0aec0" },
                },
                invalid: { color: "#fa755a" },
              },
            }}
          />
          <button
            type="submit"
            className="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded w-full"
          >
            Pay Now
          </button>
        </form>
      </div>
    </div>
  );
};

const Payment = ({ showCardPopup, selectedItem, handleCardPopup }) => {
  const modalRoot = document.getElementById("payment-modal-root");
  if (!showCardPopup || !modalRoot) return null;

  return ReactDOM.createPortal(
    <Elements stripe={stripePromise}>
      <PaymentModal selectedItem={selectedItem} handleCardPopup={handleCardPopup} />
    </Elements>,
    modalRoot
  );
};

export default Payment;
