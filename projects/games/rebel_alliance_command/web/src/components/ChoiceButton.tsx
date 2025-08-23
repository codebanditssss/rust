import React from 'react';
import { GameOption } from '../types';

interface ChoiceButtonProps {
  option: GameOption;
  onClick: (id: number) => void;
}

const ChoiceButton: React.FC<ChoiceButtonProps> = ({ option, onClick }) => {
  const handleClick = () => {
    if (option.available) {
      onClick(option.id);
    }
  };

  return (
    <button
      className="choice-button"
      onClick={handleClick}
      disabled={!option.available}
      title={option.requirement || option.description}
    >
      <div className="choice-icon">{option.icon}</div>
      <div className="choice-text">{option.text}</div>
      <div className="choice-description">{option.description}</div>
      
      {option.cost && (
        <div className="choice-cost">
          💰 Cost: {option.cost} {option.cost === 1 ? 'credit' : 'credits'}
        </div>
      )}
      
      {!option.available && option.requirement && (
        <div className="choice-requirement">
          ⚠️ {option.requirement}
        </div>
      )}
    </button>
  );
};

export default ChoiceButton;
