import '@testing-library/jest-dom';
import { render, screen } from '@testing-library/react';
import Home from '../src/app/page';

xdescribe('Home', () => {
  it('タイトルを表示する', () => {
    render(<Home searchParams={{ name: 'user1' }} />);

    const heading = screen.getByRole('heading', { level: 1 });

    expect(heading.textContent).toBe('Hello, World!');
  });

  it('スナップショットテスト', () => {
    const { container } = render(<Home searchParams={{ name: 'user1' }} />);
    expect(container).toMatchSnapshot();
  });
});
