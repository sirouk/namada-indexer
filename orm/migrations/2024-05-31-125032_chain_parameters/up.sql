CREATE TABLE chain_parameters (
  epoch INT PRIMARY KEY,
  unbonding_length INT NOT NULL,
  pipeline_length INT NOT NULL
);