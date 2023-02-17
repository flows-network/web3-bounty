import { Alert as A } from 'react-bootstrap';

export default function Alert({params, hide}: any) {
  setTimeout(() => {
    hide();
  }, 5000);
  return (
    <div className="position-absolute" style={{top: 10}}>
      <A variant={params.variant} onClose={() => hide()} dismissible>
        {params.message}
      </A>
    </div>
  );
}

