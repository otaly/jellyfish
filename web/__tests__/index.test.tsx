import '@testing-library/jest-dom';
import { render, screen } from '@testing-library/react';
import Home from '../src/app/page';

describe('Home', () => {
  it('タイトルを表示する', () => {
    render(<Home />);

    const heading = screen.getByRole('heading', { level: 1 });

    expect(heading.textContent).toBe('Hello, World!');
  });

  it('スナップショットテスト', () => {
    const { container } = render(<Home />);
    expect(container).toMatchSnapshot();
  });
});
