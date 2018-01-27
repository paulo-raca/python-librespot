use librespot;
use SpotifyId;
use cpython::{PyResult, PyObject, Python, ObjectProtocol, PyBytes};
use pyfuture::py_wrap_future;
use tokio_core::reactor::Remote;

py_class!(pub class Player |py| {
    data player : librespot::player::Player;
    data handle: Remote;

    def load(&self, track: SpotifyId, play: bool = true, position_ms: u32 = 0) -> PyResult<PyObject> {
        let player = self.player(py);
        let handle = self.handle(py).clone();
        let track = *track.id(py);

        let end_of_track = player.load(track, play, position_ms);
        py_wrap_future(py, handle, end_of_track, |_py, _result| {
            Ok(true)
        })
    }

    def play(&self) -> PyResult<PyObject> {
        let player = self.player(py);
        player.play();
        Ok(py.None())
    }

    def pause(&self) -> PyResult<PyObject> {
        let player = self.player(py);
        player.pause();
        Ok(py.None())
    }

    def stop(&self) -> PyResult<PyObject> {
        let player = self.player(py);
        player.stop();
        Ok(py.None())
    }
});

impl Player {
    pub fn new(py: Python, session: librespot::core::session::Session, device: PyObject, handle: Remote) -> PyResult<Player> {
        use librespot::core::config::PlayerConfig;
        
        let config = PlayerConfig::default();

        println!("LibRespot: device = {:?}", device);
        let device_type = device.get_type(py);
        let type_name = device_type.name(py);
        println!("LibRespot: Device Type obj {:?}", type_name);

        let backend = librespot::audio_backend::dynamic_sink();

        // Uses default backend: Pipe
        // let backend = librespot::audio_backend::find(None).unwrap();
        
        let player = librespot::player::Player::new(config, session, None, move || {
            // Argument to sink builder is Option<String>
            (backend)(move |x: &[u8]| {
                {
                    let guard = Python::acquire_gil();
                    let py = guard.python();
                    let python_bytes = PyBytes::new(py, x);
                    // TODO: Should check this error!
                    device.call_method(py, "write", (python_bytes,), None).unwrap();
                }
                Ok(())
            })
            // (backend)(None)
        });

        Player::create_instance(py, player, handle)
    }
}

