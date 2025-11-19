import type { TodoEvent } from '../po';

const TodoLabel = ({ todoEvent }: { todoEvent: TodoEvent }) => {
  switch (todoEvent) {
    case 'TODO':
      return <span className=" px-2 bg-blue-300 text-black">{todoEvent}</span>;
    case 'DONE':
      return <span className=" px-2 bg-gray-300 text-black">{todoEvent}</span>;

    case 'WAIT':
      return <span className=" px-2 bg-purple-300 text-black">{todoEvent}</span>;
    case 'CANCEL':
      return <span className=" px-2 bg-gray-500 text-black">{todoEvent}</span>;
    case 'DOING':
      return <span className=" px-2 bg-red-300 text-black">{todoEvent}</span>;
  }
};

export default TodoLabel;
