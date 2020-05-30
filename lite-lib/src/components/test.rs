

fn main(&self, state: &State) {
  if let Some(dispatch) = &self.map_dispatch_to_props {
      dispatch(state);
  }
}